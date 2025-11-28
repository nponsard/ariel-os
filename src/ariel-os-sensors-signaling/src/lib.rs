#![no_std]

use core::{
    cell::Cell,
    future::poll_fn,
    pin::Pin,
    task::{Context, Poll, Waker},
};

use embassy_sync::blocking_mutex::{Mutex, raw::CriticalSectionRawMutex};

#[derive(Debug)]
enum ReadingState<T> {
    None,
    WaitingReading(Waker),
    ReadingReady(T),
    WaitingSend(T, Waker),
}

#[derive(Debug)]
enum TriggerState {
    None,
    Waiting(Waker),
    Signaled,
}

struct SensorSignalingState<T> {
    pub trigger_state: TriggerState,
    pub reading_state: ReadingState<T>,
}

impl<T> Default for SensorSignalingState<T> {
    fn default() -> Self {
        Self {
            trigger_state: TriggerState::None,
            reading_state: ReadingState::None,
        }
    }
}
// TODO: rename this
pub struct SensorSignaling<T> {
    inner: Mutex<CriticalSectionRawMutex, Cell<SensorSignalingState<T>>>,
}

impl<T> Default for SensorSignaling<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SensorSignaling<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(Cell::new(SensorSignalingState::<T> {
                trigger_state: TriggerState::None,
                reading_state: ReadingState::<T>::None,
            })),
        }
    }

    pub fn trigger_measurement(&self) {
        self.inner.lock(|inner| {
            let state = inner.replace({
                SensorSignalingState::<T> {
                    trigger_state: TriggerState::Signaled,
                    // Remove the possibly lingering reading.
                    reading_state: ReadingState::<T>::None,
                }
            });

            if let TriggerState::Waiting(waker) = state.trigger_state {
                waker.wake();
            }
        });
    }

    fn poll_wait_trigger(&self, cx: &mut Context<'_>) -> Poll<()> {
        self.inner.lock(|cell| {
            let state = cell.take();
            let reading_state = state.reading_state;
            match state.trigger_state {
                TriggerState::None => {
                    cell.set(SensorSignalingState {
                        trigger_state: TriggerState::Waiting(cx.waker().clone()),
                        reading_state,
                    });
                    Poll::Pending
                }
                TriggerState::Waiting(w) if w.will_wake(cx.waker()) => {
                    cell.set(SensorSignalingState {
                        trigger_state: TriggerState::Waiting(w),
                        reading_state,
                    });

                    Poll::Pending
                }
                TriggerState::Waiting(w) => {
                    cell.set(SensorSignalingState {
                        trigger_state: TriggerState::Waiting(cx.waker().clone()),
                        reading_state,
                    });
                    w.wake();
                    Poll::Pending
                }
                TriggerState::Signaled => {
                    cell.set(SensorSignalingState {
                        trigger_state: TriggerState::None,
                        reading_state,
                    });

                    Poll::Ready(())
                }
            }
        })
    }

    pub async fn wait_for_trigger(&self) {
        poll_fn(move |cx| self.poll_wait_trigger(cx)).await;
    }

    fn poll_send_reading(&self, cx: &mut Context<'_>, res: T) -> (Option<T>, Poll<()>) {
        self.inner.lock(|cell| {
            let state = cell.take();
            let trigger_state = state.trigger_state;

            match state.reading_state {
                ReadingState::None => {
                    cell.set(SensorSignalingState {
                        trigger_state,
                        reading_state: ReadingState::ReadingReady(res),
                    });
                    (None, Poll::Ready(()))
                }
                ReadingState::ReadingReady(prev) => {
                    cell.set(SensorSignalingState {
                        trigger_state,
                        reading_state: ReadingState::WaitingSend(prev, cx.waker().clone()),
                    });
                    (Some(res), Poll::Pending)
                }
                ReadingState::WaitingReading(read_waker) => {
                    cell.set(SensorSignalingState {
                        trigger_state,
                        reading_state: ReadingState::ReadingReady(res),
                    });
                    read_waker.wake();
                    (None, Poll::Ready(()))
                }
                ReadingState::WaitingSend(prev, prev_waker) => {
                    if prev_waker.will_wake(cx.waker()) {
                        cell.set(SensorSignalingState {
                            trigger_state,
                            // Optimization from WakerRegistration:
                            // "If both the old and new Wakers wake the same task, we can simply
                            // keep the old waker, skipping the clone."
                            reading_state: ReadingState::WaitingSend(prev, prev_waker),
                        });
                    } else {
                        cell.set(SensorSignalingState {
                            trigger_state,
                            reading_state: ReadingState::WaitingSend(prev, cx.waker().clone()),
                        });

                        // We can't store multiple wakers, they will fight eachother until the data
                        // is read once.
                        // This should happen only if a measurement is triggered multiple times and
                        // the value is not read.
                        prev_waker.wake();
                    }
                    (Some(res), Poll::Pending)
                }
            }
        })
    }

    pub async fn send_reading(&self, res: T) {
        SensorSignalingSendFuture {
            message: Some(res),
            signaling: self,
        }
        .await;
    }

    fn poll_receive_reading(&self, cx: &mut Context<'_>) -> Poll<T> {
        self.inner.lock(|cell| {
            let state = cell.take();
            let trigger_state = state.trigger_state;
            match state.reading_state {
                ReadingState::None => {
                    cell.set(SensorSignalingState {
                        trigger_state,
                        reading_state: ReadingState::WaitingReading(cx.waker().clone()),
                    });
                    Poll::Pending
                }
                ReadingState::WaitingReading(prev_waker) => {
                    if prev_waker.will_wake(cx.waker()) {
                        cell.set(SensorSignalingState {
                            trigger_state,
                            // Optimization from WakerRegistration:
                            // "If both the old and new Wakers wake the same task, we can simply
                            // keep the old waker, skipping the clone."
                            reading_state: ReadingState::WaitingReading(prev_waker),
                        });
                    } else {
                        cell.set(SensorSignalingState {
                            trigger_state,
                            reading_state: ReadingState::WaitingReading(cx.waker().clone()),
                        });

                        // We can't store multiple wakers, they will fight eachother until the data
                        // is read once.
                        // This should happen only if a measurement is triggered multiple times and
                        // the value is not read.
                        prev_waker.wake();
                    }
                    Poll::Pending
                }
                ReadingState::ReadingReady(res) => {
                    cell.set(SensorSignalingState {
                        trigger_state,
                        reading_state: ReadingState::None,
                    });
                    Poll::Ready(res)
                }
                ReadingState::WaitingSend(res, waker) => {
                    cell.set(SensorSignalingState {
                        trigger_state,
                        reading_state: ReadingState::None,
                    });
                    waker.wake();
                    Poll::Ready(res)
                }
            }
        })
    }

    pub fn receive_reading(&'static self) -> SensorSignalingReceiveFuture<'static, T> {
        SensorSignalingReceiveFuture { signaling: self }
    }
}

#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct SensorSignalingReceiveFuture<'ch, T> {
    signaling: &'ch SensorSignaling<T>,
}

impl<T> Future for SensorSignalingReceiveFuture<'_, T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        self.signaling.poll_receive_reading(cx)
    }
}

#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct SensorSignalingSendFuture<'ch, T> {
    signaling: &'ch SensorSignaling<T>,
    message: Option<T>,
}

impl<T> Unpin for SensorSignalingSendFuture<'_, T> {}
impl<T> Future for SensorSignalingSendFuture<'_, T> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.message.take() {
            Some(msg) => {
                let (out, poll) = self.signaling.poll_send_reading(cx, msg);
                self.message = out;
                poll
            }
            None => panic!("Message cannot be None"),
        }
    }
}
