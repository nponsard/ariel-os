//! Common types for GNSS functionality across different HALs.
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    watch::{Receiver, Sender, Watch},
};
use fixed::{
    FixedI32, FixedU32,
    types::extra::{U6, U8, U19, U20, U23, U24},
};

/// Maximum number of concurrent receivers for GNSS data.
pub const MAX_WATCH_RECEIVERS: usize = 4;
/// Embassy Watch to hold the latest GNSS data and notify receivers.
pub type GnssDataWatch = Watch<CriticalSectionRawMutex, GnssData, MAX_WATCH_RECEIVERS>;
/// Type alias for the receiver of GNSS data (receiver of `GnssDataWatch`).
pub type GnssDataReceiver<'a> =
    Receiver<'a, CriticalSectionRawMutex, GnssData, MAX_WATCH_RECEIVERS>;
/// Type alias for the sender of GNSS data (sender of `GnssDataWatch`).
#[doc(hidden)]
pub type GnssDataSender<'a> = Sender<'a, CriticalSectionRawMutex, GnssData, MAX_WATCH_RECEIVERS>;

/// Operation modes for the GNSS module.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GnssOperationMode {
    /// Always keep the GNSS module active.
    Continuous,
    /// Update the GNSS fix periodically. Period is defined in seconds.
    Periodic(u16),
    /// Try to get a GNSS fix only when requested, you won't be able to get updates using `get_receiver`. Timeout is defined in seconds.
    SingleShot(u16),
}

impl core::fmt::Display for GnssOperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

/// Configuration for the GNSS.
///
/// You can customize it using the `gnss-config-override` feature.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// The mode of GNSS to use.
    pub operation_mode: GnssOperationMode,
}

impl Config {
    /// Creates a new `Config` with the specified operation mode.
    #[must_use]
    pub const fn new(operation_mode: GnssOperationMode) -> Self {
        Self { operation_mode }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(GnssOperationMode::Continuous)
    }
}

impl core::fmt::Display for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

/// Represents position data from GNSS.
#[derive(Debug, Copy, Clone)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GnssPosition {
    /// Latitude in degrees.
    /// Positive values indicate north, negative values indicate south.
    pub latitude: FixedI32<U24>, // 1 sign bit, 7 integer bits, 24 fractional bits
    /// Longitude in degrees
    /// Positive values indicate east, negative values indicate west.
    pub longitude: FixedI32<U23>, // 1 sign bit, 8 integer bits, 23 fractional bits, this makes the difference between two consecutive values at the equator of 40,075,016.7/(360*2^23) ~= 0.013 meters
    /// Altitude in meters above sea level
    pub altitude: FixedI32<U6>, // 1 sign bit, 25 integer bits, 6 fractional bits, this gives a maximum altitude of 33554 km, GPS statellites orbit at about 20,200 km. The difference between two consecutive values is 1/2^6 ~= 0.015625 meters
    /// Accuracy of the position in meters
    pub accuracy: FixedU32<U8>, // 24 integer bits, 8 fractional bits, range from around 0 to 16777215 meters, with a difference between two consecutive values of 1/2^8 ~= 0.0039 meters
    /// Altitude accuracy in meters
    pub altitude_accuracy: FixedU32<U8>,
}

#[cfg(feature = "defmt")]
impl defmt::Format for GnssPosition {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(
            fmt,
            "GnssPosition {{ latitude: {}, longitude: {}, altitude: {}, accuracy: {}, altitude_accuracy: {} }}",
            self.latitude.to_num::<f32>(),
            self.longitude.to_num::<f32>(),
            self.altitude.to_num::<f32>(),
            self.accuracy.to_num::<f32>(),
            self.altitude_accuracy.to_num::<f32>()
        );
    }
}

impl core::fmt::Display for GnssPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

/// Represents velocity data from GNSS.
#[derive(Debug, Copy, Clone)]
pub struct GnssVelocity {
    /// Speed in m/s
    pub speed: FixedU32<U20>, // 12 integer bits, 20 fractional bits. Makes max speed of around 4095 m/s, 10 time the speed of sound. Difference between two consecutive values is 1/2^20 ~= 9.54e-7 m/s
    /// Speed accuracy in m/s
    pub speed_accuracy: FixedU32<U20>,
    /// Vertical speed in m/s
    pub vertical_speed: FixedI32<U19>, // Vertical speed can be negative, losing some decimal precision (1.91e-6 m/s).
    /// Vertical speed accuracy in m/s
    pub vertical_speed_accuracy: FixedU32<U19>,
    /// Heading in degrees (0–360)
    pub heading: FixedU32<U23>, // 9 integer bits, 24 fractional bits. Difference between two consecutive values is 1/2^23 ~= 1.19e-7 degrees
    /// Heading accuracy in degrees
    pub heading_accuracy: FixedU32<U23>,
}

#[cfg(feature = "defmt")]
impl defmt::Format for GnssVelocity {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(
            fmt,
            "GnssPosition {{ speed: {}, speed_accuracy: {}, vertical_speed: {}, vertical_speed_accuracy: {}, heading: {}, heading_accuracy: {} }}",
            self.speed.to_num::<f32>(),
            self.speed_accuracy.to_num::<f32>(),
            self.vertical_speed.to_num::<f32>(),
            self.vertical_speed_accuracy.to_num::<f32>(),
            self.heading.to_num::<f32>(),
            self.heading_accuracy.to_num::<f32>()
        );
    }
}

impl core::fmt::Display for GnssVelocity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

// Based on NMEA RMC message
/// Represents date and time information from GNSS.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GnssDateTime {
    /// Year
    pub year: u16,
    /// Month (1-12)
    pub month: u8,
    /// Day of the month (1-31)
    pub day: u8,
    /// Hour (0-23)
    pub hour: u8,
    /// Minute (0-59)
    pub minute: u8,
    /// Second (0-59)
    pub second: u8,
    /// Milliseconds (0-999)
    pub milliseconds: u16,
}

impl core::fmt::Display for GnssDateTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

/// Represents GNSS data that can be received from the GNSS module.
///
/// A field can be `None` if the GNSS module did not provide that information.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GnssData {
    /// The position data, if available.
    pub position: Option<GnssPosition>,
    /// The velocity data, if available.
    pub velocity: Option<GnssVelocity>,
    /// The date and time information, if available.
    pub datetime: Option<GnssDateTime>,
    /// The time when the data was recorded (from start of the MCU).
    pub recorded_at: embassy_time::Instant,
}

impl core::fmt::Display for GnssData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}
