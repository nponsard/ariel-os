/// Categories a sensor driver can be part of.
///
/// A sensor driver may be part of multiple categories.
///
/// # For sensor driver implementors
///
/// Many MEMS sensor devices (e.g., accelerometers) include a temperature sensing element in
/// addition to their main sensing element, as temperature may slightly affect the measurement
/// results.
/// Sensor *drivers* are not under the obligation of exposing such temperature readings, even if
/// they are exposed by the sensor device.
/// They may however still be fetched by the sensor driver internally, especially to dynamically
/// compute the accuracy of the main reading returned by the sensor driver.
/// If temperature readings are not exposed by the sensor driver, the sensor driver must not be
/// considered part of a category that includes temperature ([`Category::Temperature`] or
/// [`Category::AccelerometerTemperature`] in the case of an accelerometer), even if the sensor
/// *device* does expose them.
/// One reason to *not* expose the extra readings is a lack of precision or accuracy with the extra
/// sensing element.
///
/// Sensor drivers may be part of multiple categories and should then list all of them: e.g., being
/// part of the [`Category::AccelerometerTemperature`] does *not* imply also being part of the
/// [`Category::Accelerometer`] category, and the sensor driver must list both of them.
///
/// Missing variants can be added when required.
/// Please open an issue to discuss it.
// Built upon https://doc.riot-os.org/group__drivers__saul.html#ga8f2dfec7e99562dbe5d785467bb71bbb
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Category {
    /// Accelerometer.
    Accelerometer,
    /// Accelerometer & temperature sensor.
    AccelerometerTemperature,
    /// Accelerometer & gyroscope, also known as inertial measurement unit (IMU).
    AccelerometerGyroscope,
    /// Accelerometer & gyroscope & temperature sensor, also known as inertial measurement unit (IMU).
    AccelerometerGyroscopeTemperature,
    /// Accelerometer & magnetometer & temperature sensor.
    AccelerometerMagnetometerTemperature,
    /// Ammeter (ampere meter).
    Ammeter,
    /// CO₂ gas sensor.
    Co2Gas,
    /// Color sensor.
    Color,
    /// GNSS (Global Navigation Satellite System) receiver.
    Gnss,
    /// Gyroscope.
    Gyroscope,
    /// Relative humidity sensor.
    RelativeHumidity,
    /// Relative humidity & temperature sensor.
    RelativeHumidityTemperature,
    /// Light sensor.
    Light,
    /// Magnetometer.
    Magnetometer,
    /// pH sensor.
    Ph,
    /// Pressure sensor.
    Pressure,
    /// Pressure & temperature sensor.
    PressureTemperature,
    /// Push button.
    PushButton,
    /// Temperature sensor.
    Temperature,
    /// TVOC sensor.
    Tvoc,
    /// Voltage sensor.
    Voltage,
}
