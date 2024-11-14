use crate::error::DataError;

/// A runtime checked representation of the ambient pressure compensation value used as an argument
/// for the ambient pressure compensation during continuous measurements. Accepted value range:
/// [700...1400] mBar.
#[derive(Debug, PartialEq)]
pub struct AmbientPressure(u16);

const MIN_AMBIENT_PRESSURE: u16 = 700;
const MAX_AMBIENT_PRESSURE: u16 = 1400;
const AMBIENT_PRESSURE_VAL: &str = "Ambient pressure compensation";
const PRESSURE_UNIT: &str = "mBar";

impl AmbientPressure {
    /// Returns a big endian byte representation of the ambient pressure value.
    pub const fn to_be_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for AmbientPressure {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}mBar", self.0)
    }
}

impl TryFrom<u16> for AmbientPressure {
    type Error = DataError;

    /// Converts a u16 value to an [AmbientPressure]. The value must be between 700 and 1400 in mBar.
    ///
    /// # Errors
    ///
    /// - [ValueOutOfRange](crate::error::DataError::ValueOutOfRange) if `pressure` is lower than 700 or higher than
    ///   1400 mBar.
    /// - [UseDefaultPressure](crate::error::DataError::UseDefaultPressure) if `pressure` is 0.
    fn try_from(pressure: u16) -> Result<Self, Self::Error> {
        match pressure {
            0 => Err(DataError::UseDefaultPressure),
            p if !(MIN_AMBIENT_PRESSURE..=MAX_AMBIENT_PRESSURE).contains(&p) => {
                Err(DataError::ValueOutOfRange {
                    parameter: AMBIENT_PRESSURE_VAL,
                    min: MIN_AMBIENT_PRESSURE,
                    max: MAX_AMBIENT_PRESSURE,
                    unit: PRESSURE_UNIT,
                })
            }
            _ => Ok(Self(pressure)),
        }
    }
}

/// Arguments for setting the ambient pressure compensation value.
#[derive(Debug)]
pub enum AmbientPressureCompensation {
    /// Configures ambient pressure compensation to the default value of 1013.25 mBar
    DefaultPressure,
    /// Configures ambient pressure compensation to a custom value.
    CompensationPressure(AmbientPressure),
}

impl AmbientPressureCompensation {
    /// Returns a byte representation of the ambient pressure compensation value.
    pub const fn to_be_bytes(&self) -> [u8; 2] {
        match self {
            AmbientPressureCompensation::DefaultPressure => [0x00, 0x00],
            AmbientPressureCompensation::CompensationPressure(ambient_pressure) => {
                ambient_pressure.to_be_bytes()
            }
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for AmbientPressureCompensation {
    fn format(&self, f: defmt::Formatter) {
        match self {
            AmbientPressureCompensation::DefaultPressure => defmt::write!(f, "Default Pressure"),
            AmbientPressureCompensation::CompensationPressure(pres) => {
                defmt::write!(f, "Compensation Pressure: {}mBar", pres)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_sample_works() {
        let pressure = AmbientPressure(700);
        assert_eq!(pressure.to_be_bytes(), [0x02, 0xBC]);
    }

    #[test]
    fn create_allowed_value_from_u16_works() {
        let values = [700, 1000, 1400];
        for value in values {
            assert_eq!(
                AmbientPressure::try_from(value).unwrap(),
                AmbientPressure(value)
            );
        }
    }

    #[test]
    fn create_from_u16_non_null_out_of_spec_value_errors() {
        let values = [500, 2000];
        for value in values {
            assert_eq!(
                AmbientPressure::try_from(value).unwrap_err(),
                DataError::ValueOutOfRange {
                    parameter: AMBIENT_PRESSURE_VAL,
                    min: 700,
                    max: 1400,
                    unit: PRESSURE_UNIT
                }
            );
        }
    }

    #[test]
    fn create_from_u16_null_value_errors() {
        assert_eq!(
            AmbientPressure::try_from(0).unwrap_err(),
            DataError::UseDefaultPressure
        );
    }
}
