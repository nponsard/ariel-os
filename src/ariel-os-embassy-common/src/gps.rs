pub enum GpsFixMode {
    Continuous,
    Periodic(u32), // Period in milliseconds
    SingleShot,    // Will return a fix only when requested
}

/// Configuration for the GPS.
///
/// You can customize it using the `gps-config-override` feature.
pub struct Config {
    pub mode: GpsFixMode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: GpsFixMode::Continuous,
        }
    }
}

struct GpsPosition {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub accuracy: f64,
    pub altitude_accuracy: f64,
}

struct GpsVelocity {
    pub speed: f64, // Speed in m/s
    pub speed_accuracy: f64,
    pub vertical_speed: f64, // Vertical speed in m/s
    pub vertical_speed_accuracy: f64,
    pub heading: f64, // Heading in degrees
    pub heading_accuracy: f64,
}

// Based on NMEA RMC message
struct GpsDateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub milliseconds: u16,
}

pub struct GpsData {
    position: Option<GpsPosition>,
    velocity: Option<GpsVelocity>,
    datetime: Option<GpsDateTime>,
}
