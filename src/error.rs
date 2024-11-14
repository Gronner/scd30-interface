//! Errors emitted by this library.

use embedded_hal::i2c;
use thiserror::Error;

/// Error variants emitted by this library.
#[derive(Debug, Error, PartialEq)]
pub enum Scd30Error<I2cErr: i2c::Error> {
    /// Emitted when an error handling the data has occurred.
    #[error(transparent)]
    DataError(#[from] DataError),
    /// Emitted when an error with the underlying I2C bus has occurred.
    #[error(transparent)]
    I2cError(#[from] I2cErr),
    /// Emitted when the argument intended to be sent to the sensor is bigger than 16-bits. Should
    /// only occur if modifications to this library where made that send such data.
    #[error("Only 16-bits of data can be send")]
    SentDataToBig,
}

#[cfg(feature = "defmt")]
impl<I2cErr: i2c::Error> defmt::Format for Scd30Error<I2cErr> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}", self)
    }
}

/// Error variants handling data errors.
#[derive(Debug, Error, PartialEq)]
pub enum DataError {
    /// Emitted when a value is used to construct data send to the sensor, but the value is not in
    /// the specified value's range. Adjust the argument to a value within the specified bounds.
    #[error("{parameter} must be between {min} and {max} {unit}.")]
    ValueOutOfRange {
        /// Name of the parameter
        parameter: &'static str,
        /// Lower limit of the value
        min: u16,
        /// Upper limit of the value
        max: u16,
        /// Unit of the value
        unit: &'static str,
    },
    /// Emitted when the ambient pressure compensation is set to 0. Use either no value or the
    /// [DefaultPressure](crate::data::AmbientPressureCompensation::DefaultPressure) enum variant.
    #[error("Instead of setting the ambient pressure compensation to 0, use AmbientPressureCompensation::DefaultPressure.")]
    UseDefaultPressure,
    /// Emitted when the CRC check for received data fails.
    #[error("CRC check failed.")]
    CrcFailed,
    /// Emitted when data received does not match the expected data size.
    #[error("Buffer size received to wrong size for expected data.")]
    ReceivedBufferWrongSize,
    /// Emitted when a enum value received is not within the expected value range. Could occur if
    /// the firmware of the sensor has received updates.
    #[error("Unexpected Value for {parameter}: expected {expected} got {actual}")]
    UnexpectedValueReceived {
        /// Name of the parameter
        parameter: &'static str,
        /// Description of the expected value range
        expected: &'static str,
        /// Actual value received
        actual: u16,
    },
}

#[cfg(feature = "defmt")]
impl defmt::Format for DataError {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}", self)
    }
}
