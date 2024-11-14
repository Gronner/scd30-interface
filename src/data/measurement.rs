use byteorder::{BigEndian, ByteOrder};

use crate::{error::DataError, util::check_deserialization};

/// A measurement read from the SCD30.
#[derive(Debug)]
pub struct Measurement {
    /// The CO2 concentration in ppm, ranging from 0 to 10.000 ppm.
    pub co2_concentration: f32,
    /// The ambient temperature in °C, ranging from -40 to 125 °C.
    pub temperature: f32,
    /// The relative humidity in %, ranging from 0 to 100 %.
    pub humidity: f32,
}

#[cfg(feature = "defmt")]
impl defmt::Format for Measurement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "{}ppm, {}°C, {}%",
            self.co2_concentration,
            self.temperature,
            self.humidity
        )
    }
}

impl TryFrom<&[u8]> for Measurement {
    type Error = DataError;

    /// Converts buffered data to a [Measurement] value.
    ///
    /// # Errors
    ///
    /// - [ReceivedBufferWrongSize](crate::error::DataError::ReceivedBufferWrongSize) if the `data` buffer is not big enough for the data
    ///   that should have been received.
    /// - [CrcFailed](crate::error::DataError::CrcFailed) if the CRC of the received data does not match.
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        check_deserialization(data, 18)?;
        Ok(Self {
            co2_concentration: f32::from_bits(BigEndian::read_u32(&[
                data[0], data[1], data[3], data[4],
            ])),
            temperature: f32::from_bits(BigEndian::read_u32(&[
                data[6], data[7], data[9], data[10],
            ])),
            humidity: f32::from_bits(BigEndian::read_u32(&[
                data[12], data[13], data[15], data[16],
            ])),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_measurement_deserializes_properly() {
        let data: [u8; 18] = [
            0x43, 0xDB, 0xCB, 0x8C, 0x2E, 0x8F, 0x41, 0xD9, 0x70, 0xE7, 0xFF, 0xF5, 0x42, 0x43,
            0xBF, 0x3A, 0x1B, 0x74,
        ];
        let result = Measurement::try_from(&data[..]).unwrap();
        assert_eq!(result.co2_concentration, 439.09515);
        assert_eq!(result.temperature, 27.23828);
        assert_eq!(result.humidity, 48.806744);
    }
}
