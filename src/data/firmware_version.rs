use crate::{error::DataError, util::check_deserialization};

/// The firmware version of the sensor.
#[derive(Clone, Copy, Debug)]
pub struct FirmwareVersion {
    /// Major version.
    pub major: u8,
    /// Minor version.
    pub minor: u8,
}

#[cfg(feature = "defmt")]
impl defmt::Format for FirmwareVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "v{}.{}", self.major, self.minor)
    }
}

impl TryFrom<&[u8]> for FirmwareVersion {
    type Error = DataError;

    /// Converts buffered data to a [FirmwareVersion] value.
    ///
    /// # Errors
    ///
    /// - [ReceivedBufferWrongSize](crate::error::DataError::ReceivedBufferWrongSize) if the `data` buffer is not big enough for the data
    ///   that should have been received.
    /// - [CrcFailed](crate::error::DataError::CrcFailed) if the CRC of the received data does not match.
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        check_deserialization(data, 3)?;
        Ok(Self {
            major: data[0],
            minor: data[1],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_specification_sample_works() {
        let data = [0x03, 0x42, 0xF3];
        let version = FirmwareVersion::try_from(&data[..]).unwrap();
        assert_eq!(version.major, 3);
        assert_eq!(version.minor, 66);
    }
}
