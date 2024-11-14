use byteorder::{BigEndian, ByteOrder};

use crate::{error::DataError, util::check_deserialization};

/// A runtime checked representation of the measurement interval configurable for the
/// continuous measurements. Accepted value range: [2...1800] s.
#[derive(Debug, PartialEq)]
pub struct MeasurementInterval(u16);

const MIN_MEASUREMENT_INTERVAL: u16 = 2;
const MAX_MEASUREMENT_INTERVAL: u16 = 1800;
const MEASUREMENT_INTERVAL_VAL: &str = "Measurement interval";
const INTERVAL_UNIT: &str = "s";

impl MeasurementInterval {
    /// Returns a big endian byte representation of the measurement interval.
    pub const fn to_be_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for MeasurementInterval {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}s", self.0)
    }
}

impl TryFrom<u16> for MeasurementInterval {
    type Error = DataError;

    /// Converts a u16 value to a [MeasurementInterval]. The value must be between 2 and 1800 in s.
    ///
    /// # Errors
    ///
    /// - [ValueOutOfRange](crate::error::DataError::ValueOutOfRange) if `interval` is lower than 2 or higher than
    ///   1800 s.
    fn try_from(interval: u16) -> Result<Self, Self::Error> {
        if !(MIN_MEASUREMENT_INTERVAL..=MAX_MEASUREMENT_INTERVAL).contains(&interval) {
            Err(DataError::ValueOutOfRange {
                parameter: MEASUREMENT_INTERVAL_VAL,
                min: MIN_MEASUREMENT_INTERVAL,
                max: MAX_MEASUREMENT_INTERVAL,
                unit: INTERVAL_UNIT,
            })
        } else {
            Ok(Self(interval))
        }
    }
}

impl TryFrom<&[u8]> for MeasurementInterval {
    type Error = DataError;

    /// Converts buffered data to a [MeasurementInterval].
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
    use super::*;

    #[test]
    fn deserialize_sample_works() {
        let data = [0x00, 0x02, 0xE3];
        let interval = MeasurementInterval::try_from(&data[..]).unwrap();
        assert_eq!(interval, MeasurementInterval(2));
    }

    #[test]
    fn serialize_sample_works() {
        let interval = MeasurementInterval(2);
        assert_eq!(interval.to_be_bytes(), [0x00, 0x02]);
    }

    #[test]
    fn create_allowed_value_from_u16_works() {
        let values = [2, 901, 1800];
        for value in values {
            assert_eq!(
                MeasurementInterval::try_from(value).unwrap(),
                MeasurementInterval(value)
            );
        }
    }

    #[test]
    fn create_from_u16_non_null_out_of_spec_value_errors() {
        let values = [1, 2000];
        for value in values {
            assert_eq!(
                MeasurementInterval::try_from(value).unwrap_err(),
                DataError::ValueOutOfRange {
                    parameter: MEASUREMENT_INTERVAL_VAL,
                    min: 2,
                    max: 1800,
                    unit: INTERVAL_UNIT
                }
            );
        }
    }
}
