/// Dummy type.
///
/// See the `OptionalPeripherals` type of your Embassy HAL crate instead.
#[doc(hidden)]
pub struct OptionalPeripherals;

/// Helper trait to support both `Peri` style and singleton style peripherals.
// NOTE: the trait is sealed as it is only used for documentation, and should never be implemented
// by users on anything.
pub trait IntoPeripheral<'a, P>: private::Sealed {
    #[must_use]
    fn into_hal_peripheral(self) -> Self;
}

#[doc(hidden)]
pub struct Peripheral;

impl private::Sealed for Peripheral {}

impl<T> IntoPeripheral<'_, T> for Peripheral {
    fn into_hal_peripheral(self) -> Peripheral {
        self
    }
}

/// Dummy type.
#[doc(hidden)]
pub struct Peripherals;

impl From<Peripherals> for OptionalPeripherals {
    fn from(_peripherals: Peripherals) -> Self {
        Self {}
    }
}

mod private {
    pub trait Sealed {}
}
