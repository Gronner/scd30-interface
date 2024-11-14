//! # SCD30 Driver
//!
//! A driver for interacting with Sensirion's [SCD30](https://sensirion.com/products/catalog/SCD30)
//! CO2 measuring sensor via I2C. This driver is based on
//! [embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal/) traits.
//!
//! # Example
//!
//! ```ignore
//! use scd30_interface::Scd30;
//! use scd30_interface::data::DataStatus;
//! use esp_hal::i2c::master::{Config, I2c};
//! use dfmt;
//!
//! #[entry]
//! fn main() {
//!     let peripherals = esp_hal::init(esp_hal::Config::default());
//!
//!     let i2c = I2c::new(peripherals.I2C0, Config::default())
//!         .with_sda(peripherals.GPIO4)
//!         .with_scl(peripherals.GPIO5);
//!
//!     let sensor = Scd30::new(i2c);
//!
//!     // Read out firmware version
//!     let firmware_version = sensor.read_firmware_version().unwrap();
//!
//!     loop {
//!         while sensor.is_data_ready() != DataStatus::Ready {}
//!         let measurement = sensor.read_measurement().unwrap();
//!         dfmt::log!("{}", measurement);
//!     }
//! }
//! ```

#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod command;
pub mod data;
pub mod error;
mod interface;
mod util;

pub use interface::Scd30;
