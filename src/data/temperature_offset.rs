use byteorder::{BigEndian, ByteOrder};

use crate::{error::DataError, util::check_deserialization};

const MIN_TEMPERATURE_OFFSET: f32 = 0.0;
const MAX_TEMPERATURE_OFFSET: f32 = 0.1 * u16::MAX as f32;
const TEMPERATURE_OFFSET_VAL: &str = "Temperature offset";
const TEMPERATURE_UNIT: &str = "°C";

/// A runtime checked representation of the forced recalibration value. Accepted value range:
/// [0.0...6553.5] °C.
#[derive(Debug, PartialEq)]
pub struct TemperatureOffset(u16);

impl TemperatureOffset {
    /// Returns a big endian byte representation of the temperature offset.
    pub const fn to_be_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for TemperatureOffset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}°C", self.0 as f32 / 100.0)
    }
}

impl TryFrom<f32> for TemperatureOffset {
    type Error = DataError;

    /// Converts a f32 value to a [TemperatureOffset]. The value must be between 0.0 and
    /// 6553.5 in °C.
    ///
    /// # Errors
    ///
    /// - [ValueOutOfRange](crate::error::DataError::ValueOutOfRange) if `offset` is lower than 0.0 or higher than
    ///   6553.5 °C.
    fn try_from(offset: f32) -> Result<Self, Self::Error> {
        if !(MIN_TEMPERATURE_OFFSET..=MAX_TEMPERATURE_OFFSET).contains(&offset) {
            Err(DataError::ValueOutOfRange {
                parameter: TEMPERATURE_OFFSET_VAL,
                min: MIN_TEMPERATURE_OFFSET as u16,
                max: (MAX_TEMPERATURE_OFFSET * 100.0) as u16,
                unit: TEMPERATURE_UNIT,
            })
        } else {
            Ok(Self((offset * 100.0) as u16))
        }
    }
}

impl TryFrom<f64> for TemperatureOffset {
    type Error = DataError;

    /// Converts a f64 value to a [TemperatureOffset]. The value must be between 0.0 and
    /// 6553.5 in °C.
    ///
    /// # Errors
    ///
    /// - [ValueOutOfRange](crate::error::DataError::ValueOutOfRange) if `offset` is lower than 0.0 or higher than
    ///   6553.5 °C.
    fn try_from(offset: f64) -> Result<Self, Self::Error> {
        if MIN_TEMPERATURE_OFFSET as f64 > offset || offset > MAX_TEMPERATURE_OFFSET as f64 {
            Err(DataError::ValueOutOfRange {
                parameter: TEMPERATURE_OFFSET_VAL,
                min: MIN_TEMPERATURE_OFFSET as u16,
                max: (MAX_TEMPERATURE_OFFSET * 100.0) as u16,
                unit: TEMPERATURE_UNIT,
            })
        } else {
            Ok(Self((offset * 100.0) as u16))
        }
    }
}

impl TryFrom<&[u8]> for TemperatureOffset {
    type Error = DataError;

    /// Converts buffered data to a [TemperatureOffset] value.
    ///
    /// # Errors
    ///
    /// - [ReceivedBufferWrongSize](crate::error::DataError::ReceivedBufferWrongSize) if the `data` buffer is not big enough for the data
    ///   that should have been received.
    /// - [CrcFailed](crate::error::DataError::CrcFailed) if the CRC of the received data does not match.
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        check_deserialization(data, 3)?;
        Ok(Self(BigEndian::read_u16(&data[..2])))
    }
}

#[cfg(test)]
mod tests {
    use core::u16;

    use super::*;

    #[test]
    fn deserialize_sample_works() {
        let data = [0x01, 0xF4, 0x33];
        let offset = TemperatureOffset::try_from(&data[..]).unwrap();
        assert_eq!(offset, TemperatureOffset(500));
    }

    #[test]
    fn serialize_sample_works() {
        let offset = TemperatureOffset(500);
        assert_eq!(offset.to_be_bytes(), [0x01, 0xF4]);
    }

    #[test]
    fn create_allowed_value_from_f32_works() {
        let values = [(0.0f32, 0), (0.1, 10), (10.0, 1000), (6553.5, u16::MAX)];
        for (value, result) in values {
            assert_eq!(
                TemperatureOffset::try_from(value).unwrap(),
                TemperatureOffset(result)
            );
        }
    }
    #[test]
    fn create_allowed_value_from_f64_works() {
        let values = [(0.0, 0), (0.1, 10), (10.0, 1000), (6553.5, u16::MAX)];
        for (value, result) in values {
            assert_eq!(
                TemperatureOffset::try_from(value).unwrap(),
                TemperatureOffset(result)
            );
        }
    }

    #[test]
    fn create_from_f32_non_null_out_of_spec_value_errors() {
        let values = [-0.1f32, 6554.0];
        for value in values {
            assert_eq!(
                TemperatureOffset::try_from(value).unwrap_err(),
                DataError::ValueOutOfRange {
                    parameter: TEMPERATURE_OFFSET_VAL,
                    min: 0,
                    max: u16::MAX,
                    unit: TEMPERATURE_UNIT
                }
            );
        }
    }

    #[test]
    fn create_from_f64_non_null_out_of_spec_value_errors() {
        let values = [-0.1, 6554.0];
        for value in values {
            assert_eq!(
                TemperatureOffset::try_from(value).unwrap_err(),
                DataError::ValueOutOfRange {
                    parameter: TEMPERATURE_OFFSET_VAL,
                    min: 0,
                    max: u16::MAX,
                    unit: TEMPERATURE_UNIT
                }
            );
        }
    }
}
