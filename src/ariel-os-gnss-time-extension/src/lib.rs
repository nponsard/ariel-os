//! # GNSS Time Extension
//!
//! This extension allows you to read the time information from Samples produced
//! by a compatible GNSS driver.
//!
//! # For implementors
//!
//! You need to return a channel with the label [`Label::OpaqueGnssTime`] containing the first
//! element followed by a channel with the label [`Label::Opaque`] containing the second element.

use ariel_os_sensors::{
    Label, Reading as _,
    sensor::{ReadingChannel, Sample, SampleError, Samples},
};
use time::{UtcDateTime, macros::utc_datetime};

const ARIEL_EPOCH: UtcDateTime = utc_datetime!(2024-01-01 0:00);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GnssTimeExtError {
    /// Error reading value from sample, see inner error.
    Reading(SampleError),
    /// Sensor does not provide the correct channels
    InvalidSensor,
}

pub trait GnssTimeExt {
    /// Returns the time of the last fix in seconds since UNIX epoch.
    fn time_of_fix_seconds(&self) -> Result<i64, GnssTimeExtError>;

    /// Returns the time of the last fix in milliseconds since UNIX epoch.
    fn time_of_fix(&self) -> Result<i64, GnssTimeExtError> {
        Ok(self.time_of_fix_seconds()? * 1000 + self.time_of_fix_delta_ms()?)
    }
    /// Returns the milliseconds part of the time of the last fix.
    fn time_of_fix_delta_ms(&self) -> Result<i64, GnssTimeExtError>;
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

/// TODO: expand documentation
///
/// The channel with the label [`Label::OpaqueGnssTime`] is the first part of the time data, seconds since ariel epoch
/// The channel following is the second part of the time data and should have the label [`Label::Opaque`], ms since last second.
///
impl GnssTimeExt for Samples {
    fn time_of_fix_seconds(&self) -> Result<i64, GnssTimeExtError> {
        let sample = get_element_after_marker(self.samples(), Label::OpaqueGnssTime, 0)
            .ok_or(GnssTimeExtError::InvalidSensor)?;

        let since_ariel_epoch: i64 = sample.1.value().map_err(GnssTimeExtError::Reading)?.into();
        Ok(ARIEL_EPOCH.unix_timestamp() + since_ariel_epoch)
    }
    fn time_of_fix_delta_ms(&self) -> Result<i64, GnssTimeExtError> {
        let sample = get_element_after_marker(self.samples(), Label::OpaqueGnssTime, 1)
            .ok_or(GnssTimeExtError::InvalidSensor)?;

        Ok(sample.1.value().map_err(GnssTimeExtError::Reading)?.into())
    }
}
