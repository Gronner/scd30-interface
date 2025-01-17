use duplicate::duplicate_item;

const ADDRESS: u8 = 0x61;
const WRITE_FLAG: u8 = 0x00;
const READ_FLAG: u8 = 0x01;

// `await` replacement needs to be a callable due to the dot notation. This tricks enables that
// use case.
#[cfg(not(tarpaulin_include))]
trait Identity: Sized {
    fn identity(self) -> Self {
        core::convert::identity(self)
    }
}

impl<T: Sized> Identity for T {}

#[duplicate_item(
    feature_        module      async   await               i2c_trait                                       test_macro;
    ["blocking"]    [blocking]  []      [identity()]        [embedded_hal::i2c::I2c<Error = I2cErr>]        [test];
    ["async"]       [asynch]    [async] [await.identity()]  [embedded_hal_async::i2c::I2c<Error = I2cErr>]  [tokio::test];
)]
pub mod module {
    //! Implementation of the SCD30's interface

    #[cfg(feature=feature_)]
    mod inner {
        use crate::{
            command::Command,
            data::{
                AltitudeCompensation, AmbientPressureCompensation, AutomaticSelfCalibration,
                DataStatus, FirmwareVersion, ForcedRecalibrationValue, Measurement,
                MeasurementInterval, TemperatureOffset,
            },
            error::Scd30Error,
            interface::{Identity, ADDRESS, READ_FLAG, WRITE_FLAG},
            util::compute_crc8,
        };

        /// Interface for the [SCD30 CO2 sensor by Sensirion](https://sensirion.com/products/catalog/SCD30).
        pub struct Scd30<I2C> {
            i2c: I2C,
        }

        impl<I2C: i2c_trait, I2cErr: embedded_hal::i2c::Error> Scd30<I2C> {
            /// Create a new SCD30 interface.
            pub fn new(i2c: I2C) -> Self {
                Self { i2c }
            }

            /// Start continuous measurements.
            /// This is stored in non-volatile memory. After power-cycling the device, it will continue
            /// measuring without being send a measurement command.
            /// Additionally an AmbientPressure value can be send, to compensate for ambient pressure.
            /// Default ambient pressure is 1013.25 mBar, can be configured in the range of 700 mBar to
            /// 1400 mBar.
            pub async fn trigger_continuous_measurements(
                &mut self,
                pressure_compensation: Option<AmbientPressureCompensation>,
            ) -> Result<(), Scd30Error<I2cErr>> {
                let data = match pressure_compensation {
                    None => [0x0, 0x0],
                    Some(pres) => pres.to_be_bytes(),
                };
                self.write(Command::TriggerContinuousMeasurement, Some(&data))
                    .await
            }

            /// Stop continuous measurements.
            pub async fn stop_continuous_measurements(&mut self) -> Result<(), Scd30Error<I2cErr>> {
                self.write(Command::StopContinuousMeasurement, None).await
            }

            /// Configures the measurement interval in seconds, ranging from to 2s to 1800s.
            pub async fn set_measurement_interval(
                &mut self,
                interval: MeasurementInterval,
            ) -> Result<(), Scd30Error<I2cErr>> {
                self.write(
                    Command::SetMeasurementInterval,
                    Some(&interval.to_be_bytes()),
                )
                .await
            }

            /// Reads out the configured continuous measurement interval
            pub async fn get_measurement_interval(
                &mut self,
            ) -> Result<MeasurementInterval, Scd30Error<I2cErr>> {
                let receive = self.read::<3>(Command::SetMeasurementInterval).await?;
                Ok(MeasurementInterval::try_from(&receive[..])?)
            }

            /// Checks whether a measurement is ready for readout.
            pub async fn is_data_ready(&mut self) -> Result<DataStatus, Scd30Error<I2cErr>> {
                let receive = self.read::<3>(Command::GetDataReady).await?;
                Ok(DataStatus::try_from(&receive[..])?)
            }

            /// Reads out a [Measurement](crate::data::Measurement) from the sensor.
            pub async fn read_measurement(&mut self) -> Result<Measurement, Scd30Error<I2cErr>> {
                let receive = self.read::<18>(Command::ReadMeasurement).await?;
                Ok(Measurement::try_from(&receive[..])?)
            }

            /// Activates or deactivates automatic self-calibration.
            pub async fn set_automatic_self_calibration(
                &mut self,
                setting: AutomaticSelfCalibration,
            ) -> Result<(), Scd30Error<I2cErr>> {
                self.write(
                    Command::ActivateAutomaticSelfCalibration,
                    Some(&setting.to_be_bytes()),
                )
                .await
            }

            /// Reads out the current state of the automatic self-calibration.
            pub async fn get_automatic_self_calibration(
                &mut self,
            ) -> Result<AutomaticSelfCalibration, Scd30Error<I2cErr>> {
                let receive = self
                    .read::<3>(Command::ActivateAutomaticSelfCalibration)
                    .await?;
                Ok(AutomaticSelfCalibration::try_from(&receive[..])?)
            }

            /// Configures the forced re-calibration (FRC) value to compensate for sensor drift. The value
            /// can range from 400 ppm to 2000 ppm.
            pub async fn set_forced_recalibration(
                &mut self,
                frc: ForcedRecalibrationValue,
            ) -> Result<(), Scd30Error<I2cErr>> {
                self.write(Command::ForcedRecalibrationValue, Some(&frc.to_be_bytes()))
                    .await
            }

            /// Reads out the configured value of the forced re-calibration (FRC) value.
            pub async fn get_forced_recalibration(
                &mut self,
            ) -> Result<ForcedRecalibrationValue, Scd30Error<I2cErr>> {
                let receive = self.read::<3>(Command::ForcedRecalibrationValue).await?;
                Ok(ForcedRecalibrationValue::try_from(&receive[..])?)
            }

            /// Configures the temperature offset to compensate for self-heating electric components. The
            /// value can range from 0.0 °C to 6553.5 °C.
            pub async fn set_temperature_offset(
                &mut self,
                offset: TemperatureOffset,
            ) -> Result<(), Scd30Error<I2cErr>> {
                self.write(Command::SetTemperatureOffset, Some(&offset.to_be_bytes()))
                    .await
            }

            /// Reads out the configured temperature offset.
            pub async fn get_temperature_offset(
                &mut self,
            ) -> Result<TemperatureOffset, Scd30Error<I2cErr>> {
                let receive = self.read::<3>(Command::SetTemperatureOffset).await?;
                Ok(TemperatureOffset::try_from(&receive[..])?)
            }

            /// Configures the altitude compensation. The value can range from 0 m to 65535 m above sea
            /// level.
            pub async fn set_altitude_compensation(
                &mut self,
                altitude: AltitudeCompensation,
            ) -> Result<(), Scd30Error<I2cErr>> {
                self.write(
                    Command::SetAltitudeCompensation,
                    Some(&altitude.to_be_bytes()),
                )
                .await
            }

            /// Reads out the configured altitude compensation.
            pub async fn get_altitude_compensation(
                &mut self,
            ) -> Result<AltitudeCompensation, Scd30Error<I2cErr>> {
                let receive = self.read::<3>(Command::SetAltitudeCompensation).await?;
                Ok(AltitudeCompensation::try_from(&receive[..])?)
            }

            /// Reads out the version of the firmware deployed on the sensor.
            pub async fn read_firmware_version(
                &mut self,
            ) -> Result<FirmwareVersion, Scd30Error<I2cErr>> {
                let receive = self.read::<3>(Command::ReadFirmwareVersion).await?;
                Ok(FirmwareVersion::try_from(&receive[..])?)
            }

            /// Executes a soft reset of the sensor.
            pub async fn soft_reset(&mut self) -> Result<(), Scd30Error<I2cErr>> {
                self.write(Command::SoftReset, None).await
            }

            async fn read<const DATA_SIZE: usize>(
                &mut self,
                command: Command,
            ) -> Result<[u8; DATA_SIZE], Scd30Error<I2cErr>> {
                self.write(command, None).await?;
                let mut data = [0; DATA_SIZE];
                self.i2c.read(ADDRESS | READ_FLAG, &mut data).await?;
                Ok(data)
            }

            async fn write(
                &mut self,
                command: Command,
                data: Option<&[u8]>,
            ) -> Result<(), Scd30Error<I2cErr>> {
                let mut sent = [command.to_be_bytes()[0], command.to_be_bytes()[1], 0, 0, 0];

                let len = if let Some(data) = data {
                    if data.len() != 2 {
                        return Err(Scd30Error::SentDataToBig);
                    }
                    sent[2] = data[0];
                    sent[3] = data[1];
                    sent[4] = compute_crc8(data);
                    5
                } else {
                    2
                };
                Ok(self.i2c.write(ADDRESS | WRITE_FLAG, &sent[..len]).await?)
            }

            /// Consumes the sensor and returns the contained I2C peripheral.
            #[cfg(not(tarpaulin_include))]
            pub fn shutdown(self) -> I2C {
                self.i2c
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::data::AmbientPressure;
            use embedded_hal::i2c;
            use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

            #[test_macro]
            async fn trigger_continuous_measurements_with_ambient_pressure_compensation() {
                let expected_transactions = [I2cTransaction::write(
                    0x61 | 0x00,
                    vec![0x00, 0x10, 0x03, 0x20, 0x2A],
                )];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                sensor
                    .trigger_continuous_measurements(Some(
                        AmbientPressureCompensation::CompensationPressure(
                            AmbientPressure::try_from(800).unwrap(),
                        ),
                    ))
                    .await
                    .unwrap();
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn trigger_continuous_measurements_spec_example_with_none() {
                let expected_transactions = [I2cTransaction::write(
                    0x61 | 0x00,
                    vec![0x00, 0x10, 0x00, 0x00, 0x81],
                )];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                sensor.trigger_continuous_measurements(None).await.unwrap();
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn trigger_continuous_measurements_spec_example() {
                let expected_transactions = [I2cTransaction::write(
                    0x61 | 0x00,
                    vec![0x00, 0x10, 0x00, 0x00, 0x81],
                )];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                sensor
                    .trigger_continuous_measurements(Some(
                        AmbientPressureCompensation::DefaultPressure,
                    ))
                    .await
                    .unwrap();
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn stop_continuous_measurements_spec_example() {
                let expected_transactions = [I2cTransaction::write(0x61 | 0x00, vec![0x01, 0x04])];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                sensor.stop_continuous_measurements().await.unwrap();
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn set_measurement_interval_spec_example() {
                let expected_transactions = [I2cTransaction::write(
                    0x61 | 0x00,
                    vec![0x46, 0x00, 0x00, 0x02, 0xE3],
                )];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                sensor
                    .set_measurement_interval(MeasurementInterval::try_from(2).unwrap())
                    .await
                    .unwrap();
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn get_measurement_interval_spec_example() {
                let expected_transactions = [
                    I2cTransaction::write(0x61 | 0x00, vec![0x46, 0x00]),
                    I2cTransaction::read(0x61 | 0x01, vec![0x00, 0x02, 0xE3]),
                ];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                let interval = sensor.get_measurement_interval().await.unwrap();
                assert_eq!(interval, MeasurementInterval::try_from(2).unwrap());
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn get_ready_status_sample_works() {
                let expected_transactions = [
                    I2cTransaction::write(0x61 | 0x00, vec![0x02, 0x02]),
                    I2cTransaction::read(0x61 | 0x01, vec![0x00, 0x01, 0xB0]),
                ];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                let ready_status = sensor.is_data_ready().await.unwrap();
                assert_eq!(ready_status, DataStatus::Ready);
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn read_measurement_spec_example() {
                let expected_transactions = [
                    I2cTransaction::write(0x61 | 0x00, vec![0x03, 0x00]),
                    I2cTransaction::read(
                        0x61 | 0x01,
                        vec![
                            0x43, 0xDB, 0xCB, 0x8C, 0x2E, 0x8F, 0x41, 0xD9, 0x70, 0xE7, 0xFF, 0xF5,
                            0x42, 0x43, 0xBF, 0x3A, 0x1B, 0x74,
                        ],
                    ),
                ];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                let measurement = sensor.read_measurement().await.unwrap();
                assert_eq!(measurement.co2_concentration, 439.09515);
                assert_eq!(measurement.temperature, 27.23828);
                assert_eq!(measurement.humidity, 48.806744);
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn set_automatic_self_calibration_spec_example() {
                let expected_transactions = [I2cTransaction::write(
                    0x61 | 0x00,
                    vec![0x53, 0x06, 0x00, 0x00, 0x81],
                )];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                sensor
                    .set_automatic_self_calibration(AutomaticSelfCalibration::Inactive)
                    .await
                    .unwrap();
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn get_automatic_self_calibration_spec_example() {
                let expected_transactions = [
                    I2cTransaction::write(0x61 | 0x00, vec![0x53, 0x06]),
                    I2cTransaction::read(0x61 | 0x01, vec![0x00, 0x00, 0x81]),
                ];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                let asc = sensor.get_automatic_self_calibration().await.unwrap();
                assert_eq!(asc, AutomaticSelfCalibration::Inactive);
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn set_forced_recalibration_spec_example() {
                let expected_transactions = [I2cTransaction::write(
                    0x61 | 0x00,
                    vec![0x52, 0x04, 0x01, 0xC2, 0x50],
                )];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                sensor
                    .set_forced_recalibration(ForcedRecalibrationValue::try_from(450).unwrap())
                    .await
                    .unwrap();
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn get_forced_recalibration_spec_example() {
                let expected_transactions = [
                    I2cTransaction::write(0x61 | 0x00, vec![0x52, 0x04]),
                    I2cTransaction::read(0x61 | 0x01, vec![0x01, 0xC2, 0x50]),
                ];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                let frc = sensor.get_forced_recalibration().await.unwrap();
                assert_eq!(frc, ForcedRecalibrationValue::try_from(450).unwrap());
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn set_temperature_offset_spec_example() {
                let expected_transactions = [I2cTransaction::write(
                    0x61 | 0x00,
                    vec![0x54, 0x03, 0x01, 0xF4, 0x33],
                )];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                sensor
                    .set_temperature_offset(TemperatureOffset::try_from(5.0).unwrap())
                    .await
                    .unwrap();
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn get_temperature_offset_spec_example() {
                let expected_transactions = [
                    I2cTransaction::write(0x61 | 0x00, vec![0x54, 0x03]),
                    I2cTransaction::read(0x61 | 0x01, vec![0x01, 0xF4, 0x33]),
                ];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                let offset = sensor.get_temperature_offset().await.unwrap();
                assert_eq!(offset, TemperatureOffset::try_from(5.0).unwrap());
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn set_altitude_compensation_spec_example() {
                let expected_transactions = [I2cTransaction::write(
                    0x61 | 0x00,
                    vec![0x51, 0x02, 0x03, 0xE8, 0xD4],
                )];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                sensor
                    .set_altitude_compensation(AltitudeCompensation::try_from(1000).unwrap())
                    .await
                    .unwrap();
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn get_altitude_compensation_spec_example() {
                let expected_transactions = [
                    I2cTransaction::write(0x61 | 0x00, vec![0x51, 0x02]),
                    I2cTransaction::read(0x61 | 0x01, vec![0x03, 0xE8, 0xD4]),
                ];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                let altitude = sensor.get_altitude_compensation().await.unwrap();
                assert_eq!(altitude, AltitudeCompensation::try_from(1000).unwrap());
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn read_firmware_spec_example() {
                let expected_transactions = [
                    I2cTransaction::write(0x61 | 0x00, vec![0xD1, 0x00]),
                    I2cTransaction::read(0x61 | 0x01, vec![0x03, 0x42, 0xF3]),
                ];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                let version = sensor.read_firmware_version().await.unwrap();
                assert_eq!(version.major, 3);
                assert_eq!(version.minor, 66);
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn execute_soft_reset_spec_example() {
                let expected_transactions = [I2cTransaction::write(0x61 | 0x00, vec![0xD3, 0x04])];

                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                sensor.soft_reset().await.unwrap();
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn read_errors_on_i2c_error() {
                let expected_transactions = [
                    I2cTransaction::write(0x61 | 0x00, vec![0xD1, 0x00]),
                    I2cTransaction::read(0x61 | 0x01, vec![0x03, 0x42, 0xF3])
                        .with_error(i2c::ErrorKind::Other),
                ];
                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                let result = sensor.read::<3>(Command::ReadFirmwareVersion);
                assert_eq!(
                    result.await.unwrap_err(),
                    Scd30Error::I2cError(i2c::ErrorKind::Other)
                );
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn write_errors_on_i2c_error() {
                let expected_transactions = [I2cTransaction::write(0x61 | 0x00, vec![0xD3, 0x04])
                    .with_error(i2c::ErrorKind::Other)];
                let i2c = I2cMock::new(&expected_transactions);

                let mut sensor = Scd30::new(i2c);

                let result = sensor.write(Command::SoftReset, None);
                assert_eq!(
                    result.await.unwrap_err(),
                    Scd30Error::I2cError(i2c::ErrorKind::Other)
                );
                sensor.shutdown().done();
            }

            #[test_macro]
            async fn write_errors_on_too_big_send_data() {
                let i2c = I2cMock::new(&[]);

                let mut sensor = Scd30::new(i2c);

                let result = sensor.write(
                    Command::SetTemperatureOffset,
                    Some([0x00, 0x00, 0x00, 0x00].as_slice()),
                );
                assert_eq!(result.await.unwrap_err(), Scd30Error::SentDataToBig);
                sensor.shutdown().done();
            }
        }
    }

    #[cfg(feature=feature_)]
    pub use inner::*;
}
