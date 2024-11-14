use crate::error::DataError;
use crate::util::check_deserialization;

const ASC_VALUE: &str = "Automatic self-calibration";
const ASC_EXPECTED: &str = "0 or 1";

/// Arguments for configuring the automatic self calibration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AutomaticSelfCalibration {
    /// Active automatic self calibration
    Active = 1,
    /// Inactive automatic self calibration
    Inactive = 0,
}

#[cfg(feature = "defmt")]
impl defmt::Format for AutomaticSelfCalibration {
    fn format(&self, f: defmt::Formatter) {
        match self {
            AutomaticSelfCalibration::Active => defmt::write!(f, "Active"),
            AutomaticSelfCalibration::Inactive => defmt::write!(f, "Inactive"),
        }
    }
}

impl AutomaticSelfCalibration {
    /// Returns a big endian byte representation of the automatic self calibration value.
    pub fn to_be_bytes(&self) -> [u8; 2] {
        (*self as u16).to_be_bytes()
    }
}

impl TryFrom<&[u8]> for AutomaticSelfCalibration {
    type Error = DataError;

    /// Converts buffered data to an [AutomaticSelfCalibration] value. If `Active` if a `1` is
    /// received, `Inactive` if a `0` is received.
    ///
    /// # Errors
    ///
    /// - [ReceivedBufferWrongSize](crate::error::DataError::ReceivedBufferWrongSize) if the `data` buffer is not big enough for the data
    ///   that should have been received.
    /// - [CrcFailed](crate::error::DataError::CrcFailed) if the CRC of the received data does not match.
    /// - [UnexpectedValueReceived](crate::error::DataError::UnexpectedValueReceived) if the received value is not `0` or `1`.
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        check_deserialization(data, 3)?;
        match data[1] {
            1 => Ok(Self::Active),
            0 => Ok(Self::Inactive),
            val => Err(DataError::UnexpectedValueReceived {
                parameter: ASC_VALUE,
                expected: ASC_EXPECTED,
                actual: val as u16,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_inactive_works() {
        let asc = AutomaticSelfCalibration::Inactive;
        assert_eq!(asc.to_be_bytes(), [0x00, 0x00]);
    }

    #[test]
    fn serialize_active_works() {
        let asc = AutomaticSelfCalibration::Active;
        assert_eq!(asc.to_be_bytes(), [0x00, 0x01]);
    }

    #[test]
    fn deserialize_inactive_spec_sample_works() {
        let data = [0x00, 0x00, 0x81];
        assert_eq!(
            AutomaticSelfCalibration::try_from(&data[..]).unwrap(),
            AutomaticSelfCalibration::Inactive
        );
    }

    #[test]
    fn deserialize_active_works() {
        let data = [0x00, 0x01, 0xB0];
        assert_eq!(
            AutomaticSelfCalibration::try_from(&data[..]).unwrap(),
            AutomaticSelfCalibration::Active
        );
    }

    #[test]
    fn deserialize_out_of_specification_value_errors() {
        let data = [0x00, 0x02, 0xE3];
        assert_eq!(
            AutomaticSelfCalibration::try_from(&data[..]).unwrap_err(),
            DataError::UnexpectedValueReceived {
                parameter: ASC_VALUE,
                expected: ASC_EXPECTED,
                actual: 2
            }
        );
    }
}
