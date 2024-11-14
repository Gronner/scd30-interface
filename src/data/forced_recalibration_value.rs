use byteorder::{BigEndian, ByteOrder};

use crate::error::DataError;
use crate::util::check_deserialization;

const MIN_FRC: u16 = 400;
const MAX_FRC: u16 = 2000;
const FRC_VAL: &str = "Forced recalibration value";
const PARTICLE_UNIT: &str = "ppm";

/// A runtime checked representation of the forced recalibration value. Accepted value range:
/// [400...2000] ppm.
#[derive(Debug, PartialEq)]
pub struct ForcedRecalibrationValue(u16);

#[cfg(feature = "defmt")]
impl defmt::Format for ForcedRecalibrationValue {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}ppm", self.0)
    }
}

impl ForcedRecalibrationValue {
    /// Returns a big endian byte representation of the forced recalibration value.
    pub const fn to_be_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

impl TryFrom<u16> for ForcedRecalibrationValue {
    type Error = DataError;

    /// Converts a u16 value to a [ForcedRecalibrationValue]. The value must be between 2 and 1800 in s.
    ///
    /// # Errors
    ///
    /// - [ValueOutOfRange](crate::error::DataError::ValueOutOfRange) if `frc` is lower than 400 or higher than 2000 ppm.
    fn try_from(frc: u16) -> Result<Self, Self::Error> {
        if !(MIN_FRC..=MAX_FRC).contains(&frc) {
            Err(DataError::ValueOutOfRange {
                parameter: FRC_VAL,
                min: MIN_FRC,
                max: MAX_FRC,
                unit: PARTICLE_UNIT,
            })
        } else {
            Ok(Self(frc))
        }
    }
}

impl TryFrom<&[u8]> for ForcedRecalibrationValue {
    type Error = DataError;

    /// Converts buffered data to a [ForcedRecalibrationValue] value.
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
        let data = [0x01, 0xC2, 0x50];
        let frc = ForcedRecalibrationValue::try_from(&data[..]).unwrap();
        assert_eq!(frc, ForcedRecalibrationValue(450));
    }

    #[test]
    fn serialize_sample_works() {
        let frc = ForcedRecalibrationValue(450);
        assert_eq!(frc.to_be_bytes(), [0x01, 0xC2]);
    }

    #[test]
    fn create_allowed_value_from_u16_works() {
        let values = [400, 1200, 2000];
        for value in values {
            assert_eq!(
                ForcedRecalibrationValue::try_from(value).unwrap(),
                ForcedRecalibrationValue(value)
            );
        }
    }

    #[test]
    fn create_from_u16_non_null_out_of_spec_value_errors() {
        let values = [300, 2100];
        for value in values {
            assert_eq!(
                ForcedRecalibrationValue::try_from(value).unwrap_err(),
                DataError::ValueOutOfRange {
                    parameter: FRC_VAL,
                    min: 400,
                    max: 2000,
                    unit: PARTICLE_UNIT
                }
            );
        }
    }
}
