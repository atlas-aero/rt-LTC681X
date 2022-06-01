//! # Generic client for LTC681X battery stack monitors
//!
//! Supports all devices of LTC681X family: [LTC6813](crate::ltc6813::LTC6813), [LTC6812](crate::ltc6812::LTC6812), [LTC6811](crate::ltc6811::LTC6811) and [LTC6810](crate::ltc6810::LTC6810).
//!
//! Currently the following features are implemented:
//! * [Cell and GPIO conversion](crate::monitor#conversion)
//! * [Reading cell and GPIO voltage registers](crate::monitor#reading-registers)
//! * [Multiple devices in daisy chain](crate::monitor#multiple-devices-in-daisy-chain)
//! * [ADC status polling (SDO line method)](crate::monitor#polling)
//! * [Mapping voltages to GPIO and cell groups](crate::monitor#mapping-voltages)
//! * [Overlapping ADC measurement](crate::monitor#overlap-measurement-adol-command)
//! * [Internal device parameters measurement](crate::monitor#internal-device-parameters-adstat-command)
//!
//! # Example
//!
//! For all details see [monitor](crate::monitor) module.
//!
//! ````
//!use ltc681x::example::{ExampleCSPin, ExampleSPIBus};
//!use ltc681x::ltc6813::{CellSelection, Channel, GPIOSelection, LTC6813};
//!use ltc681x::monitor::{ADCMode, LTC681X, LTC681XClient, PollClient};
//!
//!let spi_bus = ExampleSPIBus::default();
//!let cs_pin = ExampleCSPin{};
//!
//! // LTC6813 device
//! let mut client: LTC681X<_, _, _, LTC6813, 1> = LTC681X::ltc6813(spi_bus, cs_pin)
//!     .enable_sdo_polling();
//!
//! // Starts conversion for cell group 1
//! client.start_conv_cells(ADCMode::Normal, CellSelection::Group1, true);
//!
//! // Poll ADC status
//! while !client.adc_ready().unwrap() {
//!     // Conversion is not done yet
//! }
//!
//! // Returns the value of cell group A. In case of LTC613: cell 1, 7 and 13
//! let voltages = client.read_voltages(CellSelection::Group1).unwrap();
//! assert_eq!(Channel::Cell1, voltages[0][0].channel);
//! assert_eq!(24979, voltages[0][0].voltage);
//! ````
#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "strict", deny(warnings))]

extern crate alloc;

#[cfg(feature = "example")]
pub mod example;
pub mod ltc6810;
pub mod ltc6811;
pub mod ltc6812;
pub mod ltc6813;
pub mod monitor;

pub(crate) mod commands;
pub(crate) mod pec15;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod tests;
