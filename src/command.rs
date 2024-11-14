//! SCD30 I2C Commands.

/// I2C Commands for the SCD30 according to its [interface
/// description](https://sensirion.com/media/documents/D7CEEF4A/6165372F/Sensirion_CO2_Sensors_SCD30_Interface_Description.pdf)
#[derive(Clone, Copy)]
pub enum Command {
    /// Enable continuous measurements with an ambient pressure compensation. The ambient pressure
    /// compensation is sent as an argument after the command. Setting it to 0 uses the default
    /// value of 1013.25 mBar. Accepted value range: 0 or [700...1400] in mBar.
    TriggerContinuousMeasurement = 0x0010,
    /// Stop continuous measurements.
    StopContinuousMeasurement = 0x0104,
    /// Sets the measurement interval in continuous mode. The interval is sent as an argument
    /// after the command. The initial value is 2 s. Accepted value range: [2...1800] in s. If no
    /// argument is given the value is read back.
    SetMeasurementInterval = 0x4600,
    /// Queries whether a measurement can be read from the sensor's buffer. The answer is `1` if
    /// a measurement is available, `0` otherwise.
    GetDataReady = 0x0202,
    /// If a measurement is available reads out the measurement. The measurement contains the CO2
    /// concentration in ppm, the temperature in °C and the relative humidity in %.
    ReadMeasurement = 0x0300,
    /// (De-)Activates continuous, automatic self calibration (ASC). The setting is sent as an
    /// argument after the command. Sending a `1` activates ASC, sending a `0` deactivates ASC. See
    /// the interface description for the self-calibration procedure.
    ActivateAutomaticSelfCalibration = 0x5306,
    /// Set or get the forced re-calibration value (FRC). After re-powering this returns the standard
    /// value of 400 ppm. Sending an argument after the command sets the FRC to the sent value.
    /// Accepted value range: [400...2000] ppm. If no argument is given the value is read back.
    ForcedRecalibrationValue = 0x5204,
    /// Set temperature offset caused by self-heating. The offset is sent as an argument after the
    /// command. Accepted value range: [0.1...UINT16::MAX * 0.1] in °C.
    SetTemperatureOffset = 0x5403,
    /// Set operating height over sea level. The height is sent as an argument after the command.
    /// Accepted value range: [0..UINT16::MAX] in m above sea level. If no argument is given the
    /// value is read back.
    SetAltitudeCompensation = 0x5102,
    /// Queries the firmware version of the sensor. The responses is the major.minor version.
    ReadFirmwareVersion = 0xD100,
    /// Reset the device, similar to a power-off reset, by restarting the sensor controller.
    SoftReset = 0xD304,
}

impl Command {
    /// Returns a big endian byte representation of the command.
    pub fn to_be_bytes(&self) -> [u8; 2] {
        (*self as u16).to_be_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_to_bytes_works() {
        use Command::*;
        let data = [
            (TriggerContinuousMeasurement, [0x00, 0x10]),
            (StopContinuousMeasurement, [0x01, 0x04]),
            (SetMeasurementInterval, [0x46, 0x00]),
            (GetDataReady, [0x02, 0x02]),
            (ReadMeasurement, [0x03, 0x00]),
            (ActivateAutomaticSelfCalibration, [0x53, 0x06]),
            (ForcedRecalibrationValue, [0x52, 0x04]),
            (SetTemperatureOffset, [0x54, 0x03]),
            (SetAltitudeCompensation, [0x51, 0x02]),
            (ReadFirmwareVersion, [0xD1, 0x00]),
            (SoftReset, [0xD3, 0x04]),
        ];

        for (command, result) in data {
            assert_eq!(command.to_be_bytes(), result);
        }
    }
}
