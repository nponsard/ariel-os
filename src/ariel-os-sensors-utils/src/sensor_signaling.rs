use ariel_os_sensors::sensor::{ReadingError, ReadingResult, ReadingWaiter, Samples};
use ariel_os_sensors_signaling::SensorSignaling;

pub struct SensorSignalingWrapper {
    inner: SensorSignaling<ReadingResult<Samples>>,
}

impl SensorSignalingWrapper {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: SensorSignaling::new(),
        }
    }

    pub fn trigger_measurement(&self) {
        self.inner.trigger_measurement();
    }

    pub async fn wait_for_trigger(&self) {
        self.inner.wait_for_trigger().await;
    }

    pub async fn signal_reading(&self, reading: Samples) {
        self.inner.send_reading(Ok(reading)).await;
    }

    pub async fn signal_reading_err(&self, reading_err: ReadingError) {
        self.inner.send_reading(Err(reading_err)).await;
    }

    pub fn wait_for_reading(&'static self) -> ReadingWaiter {
        ReadingWaiter::SpecialWaiter {
            waiter: self.inner.receive_reading(),
        }
    }
}
