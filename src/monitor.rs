//! Generic client for LTX681X device family
//!
//! This module contains a generic client which supports communication with any LTC681X device.
//!
//! # Initialization
//!
//! The [client](LTC681X) is based on a SPI bus, which implements the [embedded-hal SPI Transfer trait](<https://docs.rs/embedded-hal/latest/embedded_hal/blocking/spi/trait.Transfer.html>)
//! and contains the following two generic parameters:
//! * T: Device specific types ([DeviceTypes] trait). See [LTC6813](crate::ltc6813::LTC6813), [LTC6812](crate::ltc6812::LTC6812), [LTC6811](crate::ltc6811::LTC6811) and [LTC6810](crate::ltc6810::LTC6810)
//! * L: Number of devices in daisy chain
//!
//! ````
//! use ltc681x::example::{ExampleCSPin, ExampleSPIBus};
//! use ltc681x::ltc6812::LTC6812;
//! use ltc681x::ltc6813::LTC6813;
//! use ltc681x::monitor::LTC681X;
//!
//! // Single LTC6813 device
//! let spi_bus = ExampleSPIBus::default();
//! let cs_pin = ExampleCSPin{};
//! let client: LTC681X<_, _, _, LTC6813, 1> = LTC681X::ltc6813(spi_bus, cs_pin);
//!
//! // Three LTC6812 devices in daisy chain
//! let spi_bus = ExampleSPIBus::default();
//! let cs_pin = ExampleCSPin{};
//! let client: LTC681X<_, _, _, LTC6812, 3> = LTC681X::ltc6812(spi_bus, cs_pin);
//! ````
//!
//! # Conversion
//!
//! The following section describes starting conversion and polling mechanisms.
//!
//! ## Cell conversion
//!
//! A cell conversion is started using the [LTC681XClient::start_conv_cells](LTC681XClient#tymethod.start_conv_cells) method.
//! The method takes three arguments:
//! * **mode**: ADC frequency and filter settings, s. [ADCMode]
//! * **cells**: Group of cells to be converted, e.g. [LTC6813::CellSelection](crate::ltc6813::CellSelection)
//! * **dcp**: Allow discharging during conversion?
//!
//! ````
//!# use ltc681x::example::{ExampleCSPin, ExampleSPIBus};
//!# use ltc681x::ltc6813::{CellSelection, LTC6813};
//!# use ltc681x::monitor::{ADCMode, LTC681X, LTC681XClient};
//!#
//!# let mut  client: LTC681X<_, _, _, LTC6813, 1> = LTC681X::ltc6813(ExampleSPIBus::default(), ExampleCSPin{});
//!#
//!#
//! // Converting first cell group using normal ADC mode
//! client.start_conv_cells(ADCMode::Normal, CellSelection::Group1, true);
//!
//! // Converting all cells using fast ADC mode
//! client.start_conv_cells(ADCMode::Fast, CellSelection::All, true);
//! ````
//!
//! ## GPIO conversion
//!
//! A GPIO conversion is started using the [LTC681XClient::start_conv_gpio](LTC681XClient#tymethod.start_conv_gpio) method.
//! The method takes three arguments:
//! * **mode**: ADC frequency and filter settings, s. [ADCMode]
//! * **pins**: Group of GPIO channels to be converted, e.g. [LTC6813::GPIOSelection](crate::ltc6813::GPIOSelection)
//!
//! ````
//!# use ltc681x::example::{ExampleCSPin, ExampleSPIBus};
//!# use ltc681x::ltc6813::{GPIOSelection, LTC6813};
//!# use ltc681x::monitor::{ADCMode, LTC681X, LTC681XClient};
//!#
//!# let mut  client: LTC681X<_, _, _, LTC6813, 1> = LTC681X::ltc6813(ExampleSPIBus::default(), ExampleCSPin{});
//!#
//! // Converting second GPIO group using normal ADC mode
//! client.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group2);
//!
//! // Converting all GPIOs using fast ADC mode
//! client.start_conv_gpio(ADCMode::Fast, GPIOSelection::All);
//! ````
//!
//! ## Polling
//!
//! ADC status may be be polled using the [PollClient::adc_ready](PollClient#tymethod.adc_ready) method.
//! The following poll methods are currently supported:
//!
//! ### SDO line polling
//!
//! After entering a conversion command, the SDO line is driven low when the device is busy performing
//! conversions. SDO is pulled high when the device completes conversions.
//!
//! ````
//!# use ltc681x::example::{ExampleCSPin, ExampleSPIBus};
//!# use ltc681x::ltc6813::{GPIOSelection, LTC6813};
//!# use ltc681x::monitor::{ADCMode, LTC681X, PollClient};
//!#
//!# let spi_bus = ExampleSPIBus::default();
//!# let cs_pin = ExampleCSPin{};
//! let mut  client: LTC681X<_, _, _, LTC6813, 1> = LTC681X::ltc6813(spi_bus, cs_pin)
//!     .enable_sdo_polling();
//!
//! while !client.adc_ready().unwrap() {
//!     // ADC conversion is not finished yet
//! }
//! ````
//!
//! ## Reading registers
//!
//! The content of registers may be directly read. The client returns an array containing three u16,
//! one value for each register slot.
//!
//! ````
//!# use ltc681x::example::{ExampleCSPin, ExampleSPIBus};
//!# use ltc681x::ltc6813::{GPIOSelection, LTC6813, Register};
//!# use ltc681x::monitor::{ADCMode, LTC681X, LTC681XClient, PollClient};
//!#
//!# let spi_bus = ExampleSPIBus::default();
//!# let cs_pin = ExampleCSPin{};
//! // Single LTC613 device
//! let mut client: LTC681X<_, _, _, LTC6813, 1> = LTC681X::ltc6813(spi_bus, cs_pin);
//!
//! // Reading cell voltage register B (CVBR)
//! let cell_voltages = client.read_register(Register::CellVoltageB).unwrap();
//! // Voltage of cell 5 (CVBR2/CVBR3)
//! assert_eq!(7538, cell_voltages[0][1]);
//!
//! // Reading auxiliary voltage register A (AVAR)
//! let aux_voltages = client.read_register(Register::AuxiliaryA).unwrap();
//! // Voltage of GPIO1 (AVAR0/AVAR1)
//! assert_eq!(24979, aux_voltages[0][0]);
//! ````
//!
//! ### Multiple devices in daisy chain
//!
//! The `read_register()` method returns one array for each device in daisy chain. So the first array index addresses the device index.
//! The second index addresses the slot within the register (0, 1, 2).
//!
//! ````
//!# use ltc681x::example::{ExampleCSPin, ExampleSPIBus};
//!# use ltc681x::ltc6813::{GPIOSelection, LTC6813, Register};
//!# use ltc681x::monitor::{ADCMode, LTC681X, LTC681XClient, PollClient};
//!#
//!# let spi_bus = ExampleSPIBus::default();
//!# let cs_pin = ExampleCSPin{};
//! // Three LTC613 devices in daisy chain
//! let mut client: LTC681X<_, _, _, LTC6813, 3> = LTC681X::ltc6813(spi_bus, cs_pin);
//!
//! // Reading cell voltage register A (CVAR)
//! let cell_voltages = client.read_register(Register::CellVoltageA).unwrap();
//! // Voltage of cell 1 of third device
//! assert_eq!(24979, cell_voltages[2][0]);
//!
//! // Voltage of cell 3 of second device
//! assert_eq!(8878, cell_voltages[1][2]);
//! ````
//!
//! # Mapping voltages
//!
//! Instead of manually reading voltage registers, the client offers a convenient method for mapping
//! voltages to cell or GPIO groups.
//!
//! ````
//!# use ltc681x::example::{ExampleCSPin, ExampleSPIBus};
//!# use ltc681x::ltc6813::{CellSelection, Channel, GPIOSelection, LTC6813, Register};
//!# use ltc681x::monitor::{ADCMode, LTC681X, LTC681XClient, PollClient};
//!#
//!# let spi_bus = ExampleSPIBus::default();
//!# let cs_pin = ExampleCSPin{};
//!#
//! // LTC6813 device
//! let mut client: LTC681X<_, _, _, LTC6813, 1> = LTC681X::ltc6813(spi_bus, cs_pin);
//!
//! // Returns the value of cell group A. In case of LTC613: cell 1, 7 and 13
//! let voltages = client.read_voltages(CellSelection::Group1).unwrap();
//!
//! assert_eq!(Channel::Cell1, voltages[0][0].channel);
//! assert_eq!(24979, voltages[0][0].voltage);
//!
//! assert_eq!(Channel::Cell7, voltages[0][1].channel);
//! assert_eq!(25441, voltages[0][1].voltage);
//!
//! assert_eq!(Channel::Cell13, voltages[0][2].channel);
//! assert_eq!(25822, voltages[0][2].voltage);
//!
//! // Returns the value of GPIO group 2. In case of LTC613: GPIO2 and GPIO7
//! let voltages = client.read_voltages(GPIOSelection::Group2).unwrap();
//!
//! assert_eq!(Channel::GPIO2, voltages[0][0].channel);
//! assert_eq!(7867, voltages[0][0].voltage);
//!
//! assert_eq!(Channel::GPIO7, voltages[0][1].channel);
//! assert_eq!(7869, voltages[0][1].voltage);
//! ````
use crate::monitor::Error::TransferError;
use crate::pec15::PEC15;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::slice::Iter;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;
use heapless::Vec;

/// Poll Strategy
pub trait PollMethod<CS: OutputPin> {
    /// Handles the CS pin state after command has been sent
    fn end_command(&self, cs: &mut CS) -> Result<(), CS::Error>;
}

/// Leaves CS Low and waits until SDO goes high
pub struct SDOLinePolling {}

impl<CS: OutputPin> PollMethod<CS> for SDOLinePolling {
    fn end_command(&self, _cs: &mut CS) -> Result<(), CS::Error> {
        Ok(())
    }
}

/// No ADC polling is used
pub struct NoPolling {}

impl<CS: OutputPin> PollMethod<CS> for NoPolling {
    fn end_command(&self, cs: &mut CS) -> Result<(), CS::Error> {
        cs.set_high()
    }
}

/// ADC frequency and filtering settings
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ADCMode {
    /// 27kHz or 14kHz in case of CFGAR0=1 configuration
    Fast = 0x1,
    /// 7kHz or 3kHz in case of CFGAR0=1 configuration
    Normal = 0x2,
    /// 26Hz or 2kHz in case of CFGAR0=1 configuration
    Filtered = 0x3,
    /// 422Hz or 1kHz in case of CFGAR0=1 configuration
    Other = 0x0,
}

/// Location of a conversion voltage
pub struct RegisterAddress<T: DeviceTypes> {
    /// Either a cell or GPIO
    pub(crate) channel: T::Channel,

    /// Register which stores the voltage of the channel
    pub(crate) register: T::Register,

    /// Index within register. Each register has three slots
    pub(crate) slot: usize,
}

/// Maps register locations to cell or GPIO groups
pub trait RegisterLocator<T: DeviceTypes + 'static> {
    /// Returns the register locations of the given cell or GPIO group
    fn get_locations(&self) -> Iter<'static, RegisterAddress<T>>;
}

/// Conversion result of a single channel
#[derive(PartialEq, Debug)]
pub struct Voltage<T: DeviceTypes> {
    /// Channel of the voltage
    pub channel: T::Channel,

    /// Raw register value
    /// Real voltage: voltage * 100 uV
    pub voltage: u16,
}

impl<T: DeviceTypes> Copy for Voltage<T> {}

impl<T: DeviceTypes> Clone for Voltage<T> {
    fn clone(&self) -> Self {
        Self {
            channel: self.channel,
            voltage: self.voltage,
        }
    }
}

/// Error enum of LTC681X
#[derive(PartialEq)]
pub enum Error<B: Transfer<u8>, CS: OutputPin> {
    /// SPI transfer error
    TransferError(B::Error),

    /// Error while changing state of CS pin
    CSPinError(CS::Error),

    /// PEC checksum of returned data was invalid
    ChecksumMismatch,
}

/// Trait for casting command options to command bitmaps
pub trait ToCommandBitmap {
    /// Returns the command bitmap for the given argument.
    fn to_bitmap(&self) -> u16;
}

/// Trait for casting to constant (precomputed) commands
pub trait ToFullCommand {
    /// Returns the full command + PEC15
    fn to_command(&self) -> [u8; 4];
}

/// Converts channels (cells or GPIOs) to indexes
pub trait ChannelIndex {
    /// Returns the cell index if a cell channel, otherwise None.
    fn to_cell_index(&self) -> Option<usize>;

    /// Returns the GPIO index if a GPIO channel, otherwise None.
    fn to_gpio_index(&self) -> Option<usize>;
}

/// Converts registers to indexes
pub trait GroupedRegisterIndex {
    /// Returns a **unique** index within the register group (e.g. auxiliary registers)
    fn to_index(&self) -> usize;
}

/// ADC channel type
pub enum ChannelType {
    Cell,
    GPIO,
    Reference,
}

/// Device specific types
pub trait DeviceTypes: Send + Sync + Sized + 'static {
    /// Argument for the identification of cell groups, which depends on the exact device type.
    type CellSelection: ToCommandBitmap + RegisterLocator<Self> + Copy + Clone + Send + Sync;

    /// Argument for the identification of GPIO groups, which depends on the exact device type.
    type GPIOSelection: ToCommandBitmap + RegisterLocator<Self> + Copy + Clone + Send + Sync;

    /// Argument for register selection. The available registers depend on the device.
    type Register: ToFullCommand + GroupedRegisterIndex + Copy + Clone + Send + Sync;

    /// Available cells and GPIOs
    type Channel: ChannelIndex + Into<ChannelType> + Copy + Clone + Send + Sync;

    /// Number of battery cells supported by the device
    const CELL_COUNT: usize;

    /// Number of GPIO channels
    const GPIO_COUNT: usize;
}

/// Public LTC681X client interface
///
/// L: Number of LTC681X devices in daisy chain
pub trait LTC681XClient<T: DeviceTypes, const L: usize> {
    type Error;

    /// Starts ADC conversion of cell voltages
    ///
    /// # Arguments
    ///
    /// * `mode`: ADC mode
    /// * `cells`: Measures the given cell group
    /// * `dcp`: True if discharge is permitted during conversion
    fn start_conv_cells(&mut self, mode: ADCMode, cells: T::CellSelection, dcp: bool) -> Result<(), Self::Error>;

    /// Starts GPIOs ADC conversion
    ///
    /// # Arguments
    ///
    /// * `mode`: ADC mode
    /// * `channels`: Measures t:he given GPIO group
    fn start_conv_gpio(&mut self, mode: ADCMode, pins: T::GPIOSelection) -> Result<(), Self::Error>;

    /// Reads the values of the given register
    /// Returns one array for each device in daisy chain
    fn read_register(&mut self, register: T::Register) -> Result<[[u16; 3]; L], Self::Error>;

    /// Reads and returns the conversion result (voltages) of Cell or GPIO group
    /// Returns one vector for each device in daisy chain
    ///
    /// Vector needs to have a fixed capacity until feature [generic_const_exprs](<https://github.com/rust-lang/rust/issues/76560>) is stable
    fn read_voltages<R: RegisterLocator<T> + 'static>(
        &mut self,
        locator: R,
    ) -> Result<Vec<Vec<Voltage<T>, 18>, L>, Self::Error>
    where
        T: 'static;
}

/// Public LTC681X interface for polling ADC status
pub trait PollClient {
    type Error;

    /// Returns true if the ADC is not busy
    fn adc_ready(&mut self) -> Result<bool, Self::Error>;
}

/// Client for LTC681X IC
pub struct LTC681X<B, CS, P, T, const L: usize>
where
    B: Transfer<u8>,
    CS: OutputPin,
    P: PollMethod<CS>,
    T: DeviceTypes,
{
    /// SPI bus
    bus: B,

    /// SPI CS pin
    cs: CS,

    /// Poll method used for type state
    poll_method: P,

    device_types: PhantomData<T>,
}

impl<B, CS, T, const L: usize> LTC681X<B, CS, NoPolling, T, L>
where
    B: Transfer<u8>,
    CS: OutputPin,
    T: DeviceTypes,
{
    pub(crate) fn new(bus: B, cs: CS) -> Self {
        LTC681X {
            bus,
            cs,
            poll_method: NoPolling {},
            device_types: PhantomData,
        }
    }
}

impl<B, CS, P, T, const L: usize> LTC681XClient<T, L> for LTC681X<B, CS, P, T, L>
where
    B: Transfer<u8>,
    CS: OutputPin,
    P: PollMethod<CS>,
    T: DeviceTypes,
{
    type Error = Error<B, CS>;

    /// See [LTC681XClient::start_conv_cells](LTC681XClient#tymethod.start_conv_cells)
    fn start_conv_cells(&mut self, mode: ADCMode, cells: T::CellSelection, dcp: bool) -> Result<(), Error<B, CS>> {
        self.cs.set_low().map_err(Error::CSPinError)?;
        let mut command: u16 = 0b0000_0010_0110_0000;

        command |= (mode as u16) << 7;
        command |= cells.to_bitmap();

        if dcp {
            command |= 0b0001_0000;
        }

        self.send_command(command).map_err(Error::TransferError)?;
        self.poll_method.end_command(&mut self.cs).map_err(Error::CSPinError)
    }

    /// See [LTC681XClient::start_conv_gpio](LTC681XClient#tymethod.start_conv_gpio)
    fn start_conv_gpio(&mut self, mode: ADCMode, channels: T::GPIOSelection) -> Result<(), Error<B, CS>> {
        self.cs.set_low().map_err(Error::CSPinError)?;
        let mut command: u16 = 0b0000_0100_0110_0000;

        command |= (mode as u16) << 7;
        command |= channels.to_bitmap();

        self.send_command(command).map_err(Error::TransferError)?;
        self.poll_method.end_command(&mut self.cs).map_err(Error::CSPinError)
    }

    /// See [LTC681XClient::read_cell_voltages](LTC681XClient#tymethod.read_register)
    fn read_register(&mut self, register: T::Register) -> Result<[[u16; 3]; L], Error<B, CS>> {
        self.read_daisy_chain(register.to_command())
    }

    /// See [LTC681XClient::read_cell_voltages](LTC681XClient#tymethod.read_voltages)
    fn read_voltages<R: RegisterLocator<T> + 'static>(
        &mut self,
        locator: R,
    ) -> Result<Vec<Vec<Voltage<T>, 18>, L>, Self::Error>
    where
        T: 'static,
    {
        let mut result: Vec<Vec<Voltage<T>, 18>, L> = Vec::new();

        // One slot for each register
        // 1. index: register index
        // 2. index: device index
        // 3. index: Slot within register
        let mut register_data = [[[0u16; 3]; L]; 6];

        // Array for flagging loaded registers, 0 = not loaded, 1 = loaded
        let mut loaded_registers = [0; 6];

        // Map register data
        for device_index in 0..L {
            let _ = result.push(Vec::new());

            for address in locator.get_locations() {
                let register_index = address.register.to_index();

                // Load register if not done yet
                if loaded_registers[register_index] == 0 {
                    register_data[register_index] = self.read_register(address.register)?;
                    loaded_registers[register_index] = 1;
                }

                let voltage = Voltage {
                    channel: address.channel,
                    voltage: register_data[register_index][device_index][address.slot],
                };

                let _ = result[device_index].push(voltage);
            }
        }

        Ok(result)
    }
}

impl<B, CS, P, T, const L: usize> LTC681X<B, CS, P, T, L>
where
    B: Transfer<u8>,
    CS: OutputPin,
    P: PollMethod<CS>,
    T: DeviceTypes,
{
    /// Sends the given command. Calculates and attaches the PEC checksum
    fn send_command(&mut self, command: u16) -> Result<(), B::Error> {
        let mut data = [(command >> 8) as u8, command as u8, 0x0, 0x0];
        let pec = PEC15::calc(&data[0..2]);

        data[2] = pec[0];
        data[3] = pec[1];

        self.bus.transfer(&mut data)?;
        Ok(())
    }

    /// Send the given read command and returns the response of all devices in daisy chain
    fn read_daisy_chain(&mut self, mut command: [u8; 4]) -> Result<[[u16; 3]; L], Error<B, CS>> {
        self.cs.set_low().map_err(Error::CSPinError)?;
        self.bus.transfer(&mut command).map_err(Error::TransferError)?;

        let mut result = [[0, 0, 0]; L];
        for item in result.iter_mut().take(L) {
            *item = self.read()?;
        }

        self.cs.set_high().map_err(Error::CSPinError)?;
        Ok(result)
    }

    /// Reads a register
    fn read(&mut self) -> Result<[u16; 3], Error<B, CS>> {
        let mut command = [0xff_u8; 8];
        let result = self.bus.transfer(&mut command).map_err(TransferError)?;

        let pec = PEC15::calc(&result[0..6]);
        if pec[0] != result[6] || pec[1] != result[7] {
            return Err(Error::ChecksumMismatch);
        }

        let mut registers = [result[0] as u16, result[2] as u16, result[4] as u16];
        registers[0] |= (result[1] as u16) << 8;
        registers[1] |= (result[3] as u16) << 8;
        registers[2] |= (result[5] as u16) << 8;

        Ok(registers)
    }

    /// Enables SDO ADC polling
    ///
    /// After entering a conversion command, the SDO line is driven low when the device is busy
    /// performing conversions. SDO is pulled high when the device completes conversions.
    pub fn enable_sdo_polling(self) -> LTC681X<B, CS, SDOLinePolling, T, L> {
        LTC681X {
            bus: self.bus,
            cs: self.cs,
            poll_method: SDOLinePolling {},
            device_types: PhantomData,
        }
    }
}

impl<B, CS, T, const L: usize> PollClient for LTC681X<B, CS, SDOLinePolling, T, L>
where
    B: Transfer<u8>,
    CS: OutputPin,
    T: DeviceTypes,
{
    type Error = Error<B, CS>;

    /// Returns false if the ADC is busy
    /// If ADC is ready, CS line is pulled high
    fn adc_ready(&mut self) -> Result<bool, Self::Error> {
        let mut command = [0xff];
        let result = self.bus.transfer(&mut command).map_err(Error::TransferError)?;

        if result[0] == 0xff {
            self.cs.set_high().map_err(Error::CSPinError)?;
            return Ok(true);
        }

        Ok(false)
    }
}

impl<B: Transfer<u8>, CS: OutputPin> Debug for Error<B, CS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::TransferError(_) => f.debug_struct("TransferError").finish(),
            Error::CSPinError(_) => f.debug_struct("CSPinError").finish(),
            Error::ChecksumMismatch => f.debug_struct("ChecksumMismatch").finish(),
        }
    }
}
