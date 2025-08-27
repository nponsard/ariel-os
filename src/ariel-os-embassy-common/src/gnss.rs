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

pub trait FixedPoint32 {
    const FRAC_BITS: u8;
    const SIGNED: bool;

    fn from_bits(bits: u32) -> Self;

    fn from_f32(value: f32) -> Self
    where
        Self: Sized,
    {
        let scale = 1 << Self::FRAC_BITS;
        if Self::SIGNED {
            let scaled = (value * scale as f32).round() as i32;
            Self::from_bits(scaled as u32)
        } else {
            let scaled = (value * scale as f32).round() as u32;
            Self::from_bits(scaled)
        }
    }

    // Should be fine use `as u32` on signed types as well, as we only use the raw bits
    fn as_bits(&self) -> u32;

    fn as_be_bytes(&self) -> [u8; 4] {
        self.as_bits().to_be_bytes()
    }

    /// Possibly lossy conversion to f32
    fn as_f32(&self) -> f32 {
        let scale = 1 << Self::FRAC_BITS;
        if Self::SIGNED {
            (self.as_bits() as i32) as f32 / scale as f32
        } else {
            self.as_bits() as f32 / scale as f32
        }
    }

    fn display_format(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let integer_part = self.as_bits() >> Self::FRAC_BITS;
        let mut fractional_part = self.as_bits() & ((1 << Self::FRAC_BITS) - 1);
        if Self::SIGNED {
            write!(f, "{}", integer_part as i32)?;
        } else {
            write!(f, "{}", integer_part)?;
        }

        if fractional_part == 0 {
            return Ok(());
        }
        write!(f, ".")?;

        while fractional_part > 0 {
            // Multiply by 10 to get the next decimal digit
            fractional_part *= 10;
            let digit = fractional_part >> Self::FRAC_BITS;
            write!(f, "{}", digit)?;
            // Remove the integer part
            fractional_part &= (1 << Self::FRAC_BITS) - 1;
        }
        Ok(())
    }
}

pub trait FixedPoint16 {
    const FRAC_BITS: u8;
    const SIGNED: bool;

    fn from_bits(bits: u16) -> Self;

    fn from_f32(value: f32) -> Self {
        let scale = 1 << Self::FRAC_BITS;
        if Self::SIGNED {
            let scaled = (value * scale as f32).round() as i32;
            Self::from_bits(scaled as u32)
        } else {
            let scaled = (value * scale as f32).round() as u32;
            Self::from_bits(scaled)
        }
    }

    // Should be fine use `as u16` on signed types as well, as we only use the raw bits
    fn as_bits(&self) -> u16;

    fn as_be_bytes(&self) -> [u8; 2] {
        self.as_bits().to_be_bytes()
    }

    /// Possibly lossy conversion to f32
    fn as_f32(&self) -> f32 {
        let scale = 1 << Self::FRAC_BITS;
        if Self::SIGNED {
            (self.as_bits() as i16) as f32 / scale as f32
        } else {
            self.as_bits() as f32 / scale as f32
        }
    }

    fn display_format(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let integer_part = self.as_bits() >> Self::FRAC_BITS;
        let mut fractional_part = self.as_bits() & ((1 << Self::FRAC_BITS) - 1);
        if Self::SIGNED {
            write!(f, "{}", integer_part as i32)?;
        } else {
            write!(f, "{}", integer_part)?;
        }

        if fractional_part == 0 {
            return Ok(());
        }
        write!(f, ".")?;

        while fractional_part > 0 {
            // Multiply by 10 to get the next decimal digit
            fractional_part *= 10;
            let digit = fractional_part >> Self::FRAC_BITS;
            write!(f, "{}", digit)?;
            // Remove the integer part
            fractional_part &= (1 << Self::FRAC_BITS) - 1;
        }
        Ok(())
    }
}

/// Longitude in decimal degrees.
/// Stored in fixed-point format.
/// To initialize this struct, you need to provide a fixed-point representation of the value.
/// This fixed-point representation should have 23 fractional bits, be signed and stored in an i32.
/// 1 sign bit, 8 integer bits, 23 fractional bits, this makes the difference between two consecutive values at the equator of 40,075,016.7/(360*2^23) ~= 0.013 meters
/// Range from -180 to 180 degrees.
///
/// ## Conversion example
///     let origin = -122.41943456;
///     let fixed_origin = FixedI32::<U23>::from_num(origin);
///     let manually_fixed: i32 = (origin * (1 << 23) as f64).round() as i32;
///
///     let lon = Longitude {
///         inner: fixed_origin.to_bits(),
///     };
///     let lon2 = Longitude {
///         inner: manually_fixed,
///     };
///
pub struct Longitude {
    inner: i32,
}
impl FixedPoint32 for Longitude {
    const FRAC_BITS: u8 = 23;
    const SIGNED: bool = true;

    fn from_bits(bits: u32) -> Self {
        Self { inner: bits as i32 }
    }
    fn as_bits(&self) -> u32 {
        self.inner as u32
    }
}

impl core::fmt::Display for Longitude {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.display_format(f)
    }
}

/// Latitude in decimal degrees
/// Stored in fixed-point format.
/// To initialize this struct, you need to provide a fixed-point representation of the value.
/// This fixed-point representation should have 23 fractional bits, be signed and stored in an i32.
/// 1 sign bit, 8 integer bits, 23 fractional bits, this makes the difference between two consecutive values at the equator of 40,075,016.7/(360*2^23) ~= 0.013 meters
/// Range from -90 to 90 degrees.
pub struct Latitude {
    inner: i32,
}

impl FixedPoint32 for Latitude {
    const FRAC_BITS: u32 = 23;
    const SIGNED: bool = true;

    fn as_bits(&self) -> u32 {
        self.inner as u32
    }
    fn from_bits(bits: u32) -> Self {
        Self { inner: bits as i32 }
    }
}

/// Altitude in meters above sea level.
/// Stored in fixed-point format.
/// To initialize this struct, you need to provide a fixed-point representation of the value.
/// This fixed-point representation should have 8 fractional bits, be signed and stored in an i32.
/// 1 sign bit, 23 integer bits, 8 fractional bits, this gives a maximum altitude of ~8388 km. The difference between two consecutive values is 1/2^8 ~= 0.004 meters
pub struct Altitude {
    inner: i32,
}
impl FixedPoint32 for Altitude {
    const FRAC_BITS: u32 = 8;
    const SIGNED: bool = true;

    fn as_bits(&self) -> u32 {
        self.inner as u32
    }
    fn from_bits(bits: u32) -> Self {
        Self { inner: bits as i32 }
    }
}

/// Represents position data from GNSS.
#[derive(Debug, Copy, Clone)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GnssPosition {
    /// Longitude in decimal degrees
    /// Positive values indicate east, negative values indicate west.
    /// From -180 to 180
    pub longitude: Longitude,
    /// Latitude in decimal degrees.
    /// Positive values indicate north, negative values indicate south.
    /// From -90 to 90
    pub latitude: Latitude,
    /// Altitude in meters above sea level
    pub altitude: Altitude, // 1 sign bit, 23 integer bits, 8 fractional bits, this gives a maximum altitude of ~8388 km. The difference between two consecutive values is 1/2^8 ~= 0.004 meters
    /// Horizontal accuracy in meters, the calculation of this value depends on the GNSS module.
    /// An estimation from the HDOP can be done by multiplying the HDOP by the announced horizontal accuracy of the GNSS module.
    /// Value range between 0 and 255 meters.
    pub accuracy: FixedU16<U8>, // 8 integer bits, 8 fractional bits, range from around 0 to 255 meters, with a difference between two consecutive values of 1/2^8 ~= 0.0039 meters
    /// Altitude accuracy in meters, the calculation of this value depends on the GNSS module.
    /// An estimation from the VDOP can be done by multiplying the VDOP by the announced vertical accuracy of the GNSS module.
    /// Value range between 0 and 255 meters.
    pub altitude_accuracy: FixedU16<U8>,
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
    pub speed: FixedU16<U6>, // 10 integer bits, 6 fractional bits. Makes max speed of around 1023 m/s. Difference between two consecutive values is 1/2^6 ~= 0.0016 m/s or 0.057 km/h
    /// Speed accuracy in m/s
    pub speed_accuracy: FixedU16<U6>,
    /// Vertical speed in m/s
    pub vertical_speed: FixedI16<U6>, // Vertical speed can be negative
    /// Vertical speed accuracy in m/s
    pub vertical_speed_accuracy: FixedU16<U6>,
    /// Heading in degrees (0–360)
    pub heading: FixedU16<U7>, // 9 integer bits, 7 fractional bits. Difference between two consecutive values is 1/2^7 ~= 0.008 degrees
    /// Heading accuracy in degrees
    pub heading_accuracy: FixedU16<U7>,
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
/// Represents date and time information from GNSS. All fields are in UTC.
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
}

impl core::fmt::Display for GnssData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}
