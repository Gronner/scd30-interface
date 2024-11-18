# Rust SCD30 Driver

![Crates.io Version](https://img.shields.io/crates/v/scd30-interface?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fscd30-interface)
![docs.rs](https://img.shields.io/docsrs/scd30-interface?logo=https%3A%2F%2Fdocs.rs%2Fscd30-interface%2F1.0.0%2Fscd30_interface%2F)
![Integration Pipeline](https://github.com/Gronner/scd30-interface/actions/workflows/integration.yaml/badge.svg)
[![codecov](https://codecov.io/gh/Gronner/scd30-interface/graph/badge.svg?token=NH6UCHBL19)](https://codecov.io/gh/Gronner/scd30-interface)

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
