//! # GNSS Time Extension
//!
//! This extension allows you to read the time information from [`Samples`] produced
//! by a compatible GNSS driver.
//!
//! # For users
//!
//! Import the trait [`GnssTimeExt`], you then access the time information on [`Samples`] generated
//! by a compatible driver by using those functions:
//!
//! - [`GnssTimeExt::time_of_fix_timestamp()`]: UTC time in seconds since UNIX epoch.
//! - [`GnssTimeExt::time_of_fix_timestamp_nanos()`]: UTC time in nanoseconds since UNIX epoch.
//!   Some sensors may have worse than nanosecond precision but this function
//!   will always return nanoseconds.
//!
//!
//! # For implementors
//!
//! Get parts with [`convert_datetime_to_parts()`].
//! You need to return a channel with the label [`Label::OpaqueGnssTime`] containing the first
//! part *directly followed* by a channel with the label [`Label::Opaque`] containing the second part.
#![cfg_attr(not(any(test, context = "native")), no_std)]
#![deny(missing_docs)]

use ariel_os_sensors::{
    Label, Reading as _,
    sensor::{ReadingChannel, Sample, SampleError, Samples},
};

// 2025-01-01T00:00:00.000Z in nanoseconds
// Obtained using `date --date "2025-01-01 0:00Z" -u +%s000000`
const ARIEL_EPOCH: i64 = 1_735_689_600_000_000;

/// Error returned when trying to access the time information on [`Samples`] coming from a GNSS sensor.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GnssTimeExtError {
    /// Error reading value from sample, see inner error.
    Reading(SampleError),
    /// Sensor does not provide the correct channels.
    InvalidSensor,
}

impl core::fmt::Display for GnssTimeExtError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GnssTimeExtError::Reading(e) => write!(f, "reading error: {e}"),
            GnssTimeExtError::InvalidSensor => write!(f, "invalid sensor"),
        }
    }
}

/// Error returned when trying convert the time to two parts.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GnssTimeExtConvertError {
    /// Timestamp is either too far in the past or too far in the future.
    Overflow,
}

impl core::fmt::Display for GnssTimeExtConvertError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GnssTimeExtConvertError::Overflow => write!(f, "time conversion overflowed"),
        }
    }
}

// Only implement for Samples
mod private {
    use ariel_os_sensors::sensor::Samples;

    pub trait Sealed {}
    impl Sealed for Samples {}
}

/// Trait to use to access time information on [`Samples`] coming from a GNSS sensor.
pub trait GnssTimeExt: private::Sealed {
    /// Returns the UTC time in seconds since UNIX epoch.
    ///
    /// # Errors
    ///
    /// Returns errors if the reading is not compatible with this extension
    /// or one [`Sample`] returned an error.
    fn time_of_fix_timestamp(&self) -> Result<i64, GnssTimeExtError>;

    /// Returns the time of the last fix in nanoseconds since UNIX epoch.
    ///
    /// # Errors
    ///
    /// Returns errors if the reading is not compatible with this extension
    /// or one [`Sample`] returned an error.
    fn time_of_fix_timestamp_nanos(&self) -> Result<i128, GnssTimeExtError> {
        Ok(
            i128::from(self.time_of_fix_timestamp()?) * 1_000_000_000i128
                + i128::from(self.time_of_fix_delta_ns()?),
        )
    }
    /// Returns the nanoseconds part of the time of the last fix.
    ///
    /// # Errors
    ///
    /// Returns errors if the reading is not compatible with this extension
    /// or one [`Sample`] returned an error.
    fn time_of_fix_delta_ns(&self) -> Result<i64, GnssTimeExtError>;
}

impl GnssTimeExt for Samples {
    fn time_of_fix_timestamp(&self) -> Result<i64, GnssTimeExtError> {
        let sample = get_element_after_marker(self.samples(), Label::OpaqueGnssTime, 0)
            .ok_or(GnssTimeExtError::InvalidSensor)?;

        let since_ariel_epoch: i64 = sample.1.value().map_err(GnssTimeExtError::Reading)?.into();
        Ok(ARIEL_EPOCH + since_ariel_epoch)
    }
    fn time_of_fix_delta_ns(&self) -> Result<i64, GnssTimeExtError> {
        let sample = get_element_after_marker(self.samples(), Label::OpaqueGnssTime, 1)
            .ok_or(GnssTimeExtError::InvalidSensor)?;

        Ok(sample.1.value().map_err(GnssTimeExtError::Reading)?.into())
    }
}

/// Convert time to parts to be put into Samples.
///
/// `utc_datetime` is an UTC timestamp as nanoseconds since UNIX epoch.
///
/// # Errors
/// Returns an erro when `utc_datetime` is either too far in the past or too far in the future.
///
pub fn convert_datetime_to_parts(utc_datetime: i64) -> Result<(i32, i32), GnssTimeExtConvertError> {
    let nanoseconds_since_epoch = utc_datetime
        .checked_sub(ARIEL_EPOCH)
        .ok_or(GnssTimeExtConvertError::Overflow)?;
    let seconds_since_epoch = nanoseconds_since_epoch / 1_000_000;
    let nanoseconds = nanoseconds_since_epoch
        .checked_sub(seconds_since_epoch * 1_000_000)
        .ok_or(GnssTimeExtConvertError::Overflow)?;

    Ok((
        seconds_since_epoch
            .try_into()
            .map_err(|_| GnssTimeExtConvertError::Overflow)?,
        nanoseconds
            .try_into()
            .map_err(|_| GnssTimeExtConvertError::Overflow)?,
    ))
}

fn get_element_after_marker(
    iter: impl Iterator<Item = (ReadingChannel, Sample)>,
    marker: Label,
    position: usize,
) -> Option<(ReadingChannel, Sample)> {
    let mut peekable = iter.peekable();
    while let Some((c, _)) = peekable.peek() {
        if c.label() == Label::OpaqueGnssTime {
            break;
        }
        peekable.next();
    }
    let result = peekable.nth(position);

    // if its not an opaque channel we're doing something wrong
    if let Some((c, _)) = result
        && !(c.label() == Label::Opaque || c.label() == marker)
    {
        None
    } else {
        result
    }
}
