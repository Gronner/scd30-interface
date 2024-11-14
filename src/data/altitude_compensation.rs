use byteorder::{BigEndian, ByteOrder};

use crate::{error::DataError, util::check_deserialization};

/// Altitude compensation value ranging from 0 m to 65535 m above sea level.
#[derive(Debug, PartialEq)]
pub struct AltitudeCompensation(u16);

#[cfg(feature = "defmt")]
impl defmt::Format for AltitudeCompensation {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}m", self.0)
    }
}

impl AltitudeCompensation {
    /// Returns a big endian byte representation of the altitude compensation value.
    pub const fn to_be_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

impl From<u16> for AltitudeCompensation {
    fn from(altitude: u16) -> Self {
        Self(altitude)
    }
}

impl TryFrom<&[u8]> for AltitudeCompensation {
    type Error = DataError;

    /// Converts buffered data to an [AltitudeCompensation] value.
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
    fn deserialize_specification_sample_works() {
        let data = [0x03, 0xE8, 0xD4];
        let altitude = AltitudeCompensation::try_from(&data[..]).unwrap();
        assert_eq!(altitude.0, 1000);
    }

    #[test]
    fn serialize_specification_sample_works() {
        let altitude = AltitudeCompensation(1000);
        assert_eq!(altitude.to_be_bytes(), [0x03, 0xE8]);
    }

    #[test]
    fn creating_from_u16_works() {
        let altitude = AltitudeCompensation::from(1000);
        assert_eq!(altitude, AltitudeCompensation(1000));
    }
}
