//! Provides a sensor abstraction layer.
//!
//! # Definitions
//!
//! In the context of this abstraction:
//!
//! - A *sensor device* is a device measuring one or multiple physical quantities and reporting
//!   them as one or more digital values—we call these values *samples*.
//! - Sensor devices measuring the same physical quantity are said to be part of the same *sensor
//!   category*.
//!   A sensor device may be part of multiple sensor categories.
//! - A *measurement* is the physical operation of measuring one or several physical quantities.
//! - A *reading* is the digital result returned by a sensor device after carrying out a
//!   measurement.
//!   Samples of different physical quantities can therefore be part of the same reading.
//! - A *sensor driver* refers to a sensor device as exposed by the sensor abstraction layer.
//! - A *sensor driver instance* is an instance of a sensor driver.
//!
//! # Goals
//!
//! This abstraction has two main goals:
//!
//! - Providing a unified way of accessing the readings from all registered sensor driver instances
//!   in a homogeneous way.
//! - Making it easy and as transparent as possible to substitute a specific sensor device by a
//!   similar one from the same category.
//!
//! # Accessing sensor driver instances
//!
//! Registered sensor driver instances can be accessed using
//! [`REGISTRY::sensors()`](registry::Registry::sensors).
//! Sensor drivers implement the [`Sensor`] trait, which allows to trigger measurements and obtain
//! the resulting readings.
//!
//! # Obtaining a sensor reading
//!
//! After triggering a measurement with [`Sensor::trigger_measurement()`], a reading can be
//! obtained using [`Sensor::wait_for_reading()`].
//! It is additionally necessary to use [`Sensor::reading_channels()`] to make sense of the obtained
//! reading:
//!
//! - [`Sensor::wait_for_reading()`] returns a [`Samples`](sensor::Samples), a data "tuple"
//!   containing values returned by the sensor driver.
//! - [`Sensor::reading_channels()`] returns a [`ReadingChannels`](sensor::ReadingChannels) which
//!   indicates which physical quantity each [`Sample`] from that tuple corresponds
//!   to, using a [`Label`].
//!   For instance, this allows to disambiguate the values provided by a temperature & humidity
//!   sensor.
//!
//! To avoid handling floats, [`Sample`]s returned by [`Sensor::wait_for_reading()`]
//! are integers, and a fixed scaling value is provided in
//! [`ReadingChannel`](sensor::ReadingChannel), for each [`Sample`] returned.
//! See [`Sample`] for more details.
//!
//! # For implementors
//!
//! Sensor drivers must implement the [`Sensor`] trait.
//!
//! [`Sample`]: ariel_os_sensors::sensor::Sample

pub use ariel_os_sensors::*;
#[doc(inline)]
pub use ariel_os_sensors_registry as registry;
pub use ariel_os_sensors_registry::{REGISTRY, SENSOR_REFS};
