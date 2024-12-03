# Client for LTC681X battery stack monitors
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Crates.io](https://img.shields.io/crates/v/ltc681x.svg)](https://crates.io/crates/ltc681x)
[![Actions Status](https://github.com/pegasus-aero/rt-LTC681X/workflows/QA/badge.svg)](http://github.com/pegasus-aero/rt-LTC681X/actions)

Abstraction for LTC681X family. Supports all devices of LTC681X family: [LTC6813](https://www.analog.com/en/products/ltc6813-1.html), [LTC6812](https://www.analog.com/en/products/ltc6812-1.html), [LTC6811](https://www.analog.com/en/products/ltc6811-1.html) and [LTC6810](https://www.analog.com/en/products/ltc6810-1.html).

Currently, the following features are implemented:
 * [Cell and GPIO conversion](https://docs.rs/ltc681x/latest/ltc681x/monitor/index.html#conversion)
 * [Reading cell and GPIO voltage registers](https://docs.rs/ltc681x/latest/ltc681x/monitor/index.html#reading-registers)
 * [Multiple devices in daisy chain](https://docs.rs/ltc681x/latest/ltc681x/monitor/index.html#multiple-devices-in-daisy-chain)
 * [ADC status polling (SDO line method)](https://docs.rs/ltc681x/latest/ltc681x/monitor/index.html#polling)
 * [Mapping voltages to GPIO and cell groups](https://docs.rs/ltc681x/latest/ltc681x/monitor/index.html#mapping-voltages)
 * [Abstracted device configuration](https://docs.rs/ltc681x/latest/ltc681x/config/index.html)
 * [Overlapping ADC measurement](https://docs.rs/ltc681x/latest/ltc681x/monitor/index.html#overlap-measurement-adol-command)
 * [Internal device parameters measurement](https://docs.rs/ltc681x/latest/ltc681x/monitor/index.html#internal-device-parameters-adstat-command)

## Example
For all details see [monitor](https://docs.rs/ltc681x/latest/ltc681x/monitor/index.html) module.

````Rust
use ltc681x::example::ExampleSPIDevice;
use ltc681x::ltc6813::{CellSelection, Channel, GPIOSelection, LTC6813};
use ltc681x::monitor::{ADCMode, LTC681X, LTC681XClient, PollClient};

let spi_bus = ExampleSPIDevice::default();

// LTC6813 device
let mut client: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(spi_bus);

// Starts conversion for cell group 1
client.start_conv_cells(ADCMode::Normal, CellSelection::Group1, true);

// Wait until ADC conversion is finished or poll the status
// (s. 'Conversion time' and 'Polling' section(s) of monitor module)

// Returns the value of cell group A. In case of LTC613: cell 1, 7 and 13
let voltages = client.read_voltages(CellSelection::Group1).unwrap();
assert_eq!(Channel::Cell1, voltages[0][0].channel);
assert_eq!(24979, voltages[0][0].voltage);
 ````

## State

> :warning: The crate is still incomplete, but is under active development.

> :warning: The crate has only been tested for the LTC6813 variant. Although the protocol of the LTC681X family is essentially the same, inconsistencies are still conceivable for some variants. Practical tests + feedback with other variants are therefore welcome.

## Development

Any form of support is greatly appreciated. Feel free to create issues and PRs.
See [DEVELOPMENT](DEVELOPMENT.md) for more details.  

## License
Licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
at your option.

Each contributor agrees that his/her contribution covers both licenses.