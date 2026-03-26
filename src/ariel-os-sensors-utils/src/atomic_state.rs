use ariel_os_sensors::sensor::{Mode, SetModeError, State};
use portable_atomic::{AtomicU8, Ordering};

/// A helper to store [`State`] as an atomic.
#[derive(Default)]
pub struct AtomicState {
    state: AtomicU8,
}

impl AtomicState {
    /// Creates a new [`AtomicState`].
    #[must_use]
    pub const fn new(state: State) -> Self {
        // Make sure `State` fits into a `u8`.
        const {
            assert!(core::mem::size_of::<State>() == core::mem::size_of::<u8>());
        }

        Self {
            state: AtomicU8::new(state as u8),
        }
    }

    /// Returns the current state.
    #[expect(clippy::missing_panics_doc, reason = "cannot actually panic")]
    pub fn get(&self) -> State {
        // NOTE(no-panic): cast cannot fail because the integer value always comes from *us*
        // internally casting `State`.
        State::try_from(self.state.load(Ordering::Acquire)).unwrap()
    }

    /// Sets the current state.
    pub fn set(&self, state: State) {
        self.state.store(state as u8, Ordering::Release);
    }

    /// Sets the state from a requested mode and returns the previous state.
    ///
    /// # Errors
    ///
    /// Returns [`SetModeError::Uninitialized`] if the current state is [`State::Uninitialized`].
    #[expect(clippy::missing_panics_doc, reason = "cannot actually panic")]
    pub fn set_mode(&self, mode: Mode) -> Result<State, SetModeError> {
        let new_state = State::from(mode);

        // Set the mode if the current state is not uninitialized
        self.state
            .fetch_update(Ordering::Release, Ordering::Acquire, |s| {
                if s == State::Uninitialized as u8 {
                    None
                } else {
                    Some(new_state as u8)
                }
            })
            .map(|s| {
                // NOTE(no-panic): cast cannot fail because the integer value always comes from *us*
                // internally casting `State`.
                State::try_from(s).unwrap()
            })
            .map_err(|_| SetModeError::Uninitialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_type_sizes() {
        assert_eq!(size_of::<AtomicState>(), size_of::<u8>());
        assert_eq!(align_of::<AtomicState>(), 1);
    }

    #[test]
    fn preserve_state() {
        let state = AtomicState::new(State::Uninitialized);

        state.set(State::Enabled);
        assert_eq!(state.get(), State::Enabled);

        assert!(matches!(state.set_mode(Mode::Sleeping), Ok(State::Enabled)));
        assert_eq!(state.get(), State::Sleeping);

        assert!(matches!(state.set_mode(Mode::Enabled), Ok(State::Sleeping)));
        assert_eq!(state.get(), State::Enabled);

        assert!(matches!(state.set_mode(Mode::Sleeping), Ok(State::Enabled)));
        assert_eq!(state.get(), State::Sleeping);
    }

    #[test]
    fn uninitialized_err() {
        let state = AtomicState::new(State::Uninitialized);

        assert!(matches!(
            state.set_mode(Mode::Sleeping),
            Err(SetModeError::Uninitialized)
        ));

        assert!(matches!(
            state.set_mode(Mode::Enabled),
            Err(SetModeError::Uninitialized)
        ));

        assert!(matches!(
            state.set_mode(Mode::Disabled),
            Err(SetModeError::Uninitialized)
        ));
    }
}
