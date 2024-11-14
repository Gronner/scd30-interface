# Rust SCD30 Driver

A Rust I2C driver for [Sensirion's SCD30](https://sensirion.com/products/catalog/SCD30) CO2,
temperature and humidity sensor module. This driver is based on the
[embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal/index.html) traits.

Features:

* Full implementation of the SCD30's functionality:
    * Read out CO2, temperature and relative humidity measurement.
    * Configure altitude, temperature and ambient pressure compensation.
    * Configure automatic self-recalibration and forced recalibration value.
    * Read out data status, configuration and firmware version.
    * Execute soft reset.
* Easy to integrate into projects using [embedded-hal](https://github.com/knurling-rs/defmt) crates.
* Optional [`defmt`](https://github.com/knurling-rs/defmt) support.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT licenses ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

`SPDX-License-Identifier: Apache-2.0 OR MIT`
