use crate::{error::DataError, util::check_deserialization};

const DATA_STATUS_VALUE: &str = "Data ready status";
const DATA_STATUS_EXPECTED: &str = "0 or 1";

/// Information whether a measurement is ready or not for readout.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataStatus {
    /// Data is available.
    Ready,
    /// No data is available.
    NotReady,
}

#[cfg(feature = "defmt")]
impl defmt::Format for DataStatus {
    fn format(&self, f: defmt::Formatter) {
        match self {
            DataStatus::Ready => defmt::write!(f, "Ready"),
            DataStatus::NotReady => defmt::write!(f, "Not Ready"),
        }
    }
}

impl TryFrom<&[u8]> for DataStatus {
    type Error = DataError;

    /// Converts buffered data to an [DataStatus] value.
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
            0 => Ok(Self::NotReady),
            1 => Ok(Self::Ready),
            val => Err(DataError::UnexpectedValueReceived {
                parameter: DATA_STATUS_VALUE,
                expected: DATA_STATUS_EXPECTED,
                actual: val as u16,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_not_ready_spec_sample_works() {
        let data = [0x00, 0x00, 0x81];
        assert_eq!(
            DataStatus::try_from(&data[..]).unwrap(),
            DataStatus::NotReady
        );
    }

    #[test]
    fn deserialize_ready_works() {
        let data = [0x00, 0x01, 0xB0];
        assert_eq!(DataStatus::try_from(&data[..]).unwrap(), DataStatus::Ready);
    }

    #[test]
    fn deserialize_out_of_specification_value_errors() {
        let data = [0x00, 0x02, 0xE3];
        assert_eq!(
            DataStatus::try_from(&data[..]).unwrap_err(),
            DataError::UnexpectedValueReceived {
                parameter: DATA_STATUS_VALUE,
                expected: DATA_STATUS_EXPECTED,
                actual: 2
            }
        );
    }
}
