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
//! ### Conversion time
//!
//! Execution time of cell conversions is deterministic. The expected timing is returned as [CommandTime].
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
//! let timing = client.start_conv_cells(ADCMode::Normal, CellSelection::Group1, true).unwrap();
//!
//! // 407 us in 7kHz mode (CFGAR0=0)
//! assert_eq!(407, timing.regular);
//!
//! // 523 us in 3kHz mode (CFGAR0=1)
//! assert_eq!(523, timing.alternative);
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
//! ### Conversion time
//!
//! Execution time of GPIO conversions is deterministic. The expected timing is returned as [CommandTime].
//!
//! ````
//!# use ltc681x::example::{ExampleCSPin, ExampleSPIBus};
//!# use ltc681x::ltc6813::{GPIOSelection, LTC6813};
//!# use ltc681x::monitor::{ADCMode, LTC681X, LTC681XClient};
//!#
//!# let mut  client: LTC681X<_, _, _, LTC6813, 1> = LTC681X::ltc6813(ExampleSPIBus::default(), ExampleCSPin{});
//!#
//! // Converting second GPIO group using normal ADC mode
//! let timing = client.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group2).unwrap();
//!
//! // 788 us in 7kHz mode (CFGAR0=0)
//! assert_eq!(788, timing.regular);
//!
//! // 1000 us in 3kHz mode (CFGAR0=1)
//! assert_eq!(1000, timing.alternative);
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
//!
//! # Self-tests
//!
//! The LTC681X family supports a number of verification and fault-tests.
//!
//! ## Overlap measurement (ADOL command)
//!
//! Starting the ADC overlapping measurement and reading the results:
//! ````
//!# use ltc681x::example::{ExampleCSPin, ExampleSPIBus};
//!# use ltc681x::ltc6813::{CellSelection, LTC6813};
//!# use ltc681x::monitor::{ADCMode, LTC681X, LTC681XClient};
//!#
//!# let mut  client: LTC681X<_, _, _, LTC6813, 1> = LTC681X::ltc6813(ExampleSPIBus::default(), ExampleCSPin{});
//!#
//!#
//! client.start_overlap_measurement(ADCMode::Normal, true);
//! // [...] waiting until conversion finished
//! let data = client.read_overlap_result().unwrap();
//!
//! // Voltage of cell 7 measured by ADC2
//! assert_eq!(25441, data[0][0]);
//! // Voltage of cell 7 measured by ADC1
//! assert_eq!(7869, data[0][1]);
//! // Voltage of cell 13 measured by ADC3
//! assert_eq!(25822, data[0][2]);
//! // Voltage of cell 13 measured by ADC2
//! assert_eq!(8591, data[0][3]);
//! ````
//!
//! ## Internal device parameters (ADSTAT command)
//!
//! Measuring internal device parameters and reading the results.
//!
//! The expected execution time is returned as [CommandTime], see [command timing of cell conversion](#conversion-time) as example.
//! ````
//!# use ltc681x::example::{ExampleCSPin, ExampleSPIBus};
//!# use ltc681x::ltc6813::{CellSelection, LTC6813};
//!# use ltc681x::monitor::{ADCMode, LTC681X, LTC681XClient, StatusGroup};
//!#
//!# let mut  client: LTC681X<_, _, _, LTC6813, 1> = LTC681X::ltc6813(ExampleSPIBus::default(), ExampleCSPin{});
//!#
//!#
//! client.measure_internal_parameters(ADCMode::Normal, StatusGroup::All);
//! // [...] waiting until conversion finished
//! let data = client.read_internal_device_parameters().unwrap();
//!
//! // Sum of all voltages in uV => 75.318 V
//! assert_eq!(75_318_000, data[0].total_voltage);
//! // Die temperature in °C
//! assert_eq!("56.31578", data[0].temperature.to_string());
//! // Analog power supply voltage in uV => 3.2 V
//! assert_eq!(3_200_000, data[0].analog_power);
//! // Digital power supply voltage in uV => 5.12 V
//! assert_eq!(5_120_000, data[0].digital_power);
//! ````
use crate::config::Configuration;
use crate::monitor::Error::TransferError;
use crate::pec15::PEC15;
use core::fmt::{Debug, Display, Formatter};
use core::marker::PhantomData;
use core::slice::Iter;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;
use fixed::types::I16F16;
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
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

/// Selection of status group
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StatusGroup {
    /// Includes SC, ITMP, VA, VD
    All = 0x0,
    /// Measures the total voltage of all cells (SC)
    CellSum = 0x1,
    /// Measures the internal die temperature (ITMP)
    Temperature = 0x2,
    /// Measure the internal analog voltage supply (VA)
    AnalogVoltage = 0x3,
    /// Measures the internal digital voltage supply (VD)
    DigitalVoltage = 0x4,
}

impl ToCommandBitmap for StatusGroup {
    fn to_bitmap(&self) -> u16 {
        *self as u16
    }
}

impl ToCommandTiming for StatusGroup {
    fn to_conv_command_timing(&self, mode: ADCMode) -> CommandTime {
        match self {
            StatusGroup::All => match mode {
                ADCMode::Fast => CommandTime::new(742, 858),
                ADCMode::Normal => CommandTime::new(1_600, 2_000),
                ADCMode::Filtered => CommandTime::new(134_000, 3_000),
                ADCMode::Other => CommandTime::new(8_500, 4_800),
            },
            StatusGroup::CellSum
            | StatusGroup::Temperature
            | StatusGroup::AnalogVoltage
            | StatusGroup::DigitalVoltage => match mode {
                ADCMode::Fast => CommandTime::new(200, 229),
                ADCMode::Normal => CommandTime::new(403, 520),
                ADCMode::Filtered => CommandTime::new(34_000, 753),
                ADCMode::Other => CommandTime::new(2_100, 1_200),
            },
        }
    }
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

    /// Writing to to the given register is not supported
    ReadOnlyRegister,
}

/// Trait for casting command options to command bitmaps
pub trait ToCommandBitmap {
    /// Returns the command bitmap for the given argument.
    fn to_bitmap(&self) -> u16;
}

/// Trait for determining the estimated execution time
pub trait ToCommandTiming {
    /// Returns the expected execution time of ADCV command based on the ADC mode
    fn to_conv_command_timing(&self, mode: ADCMode) -> CommandTime;
}

/// Error in case writing to this register ist not supported and therefore no command exists.
#[derive(Debug)]
pub struct NoWriteCommandError {}

impl Display for NoWriteCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "No write command for read-only register")
    }
}

/// Trait for casting to constant (precomputed) commands
pub trait ToFullCommand {
    /// Returns the full register read command + PEC15
    fn to_read_command(&self) -> [u8; 4];

    /// Returns the full register write command + PEC15
    /// Returns error in case writing to this register is not supported
    fn to_write_command(&self) -> Result<[u8; 4], NoWriteCommandError>;
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

/// Expected execution time of the issued command
#[derive(Copy, Clone, Debug)]
pub struct CommandTime {
    /// Regular (CFGAR0=0) execution time in microseconds
    pub regular: u32,

    /// Alternative (CFGAR0=1) execution time in microseconds
    pub alternative: u32,
}

impl CommandTime {
    pub fn new(regular: u32, alternative: u32) -> Self {
        Self { regular, alternative }
    }
}

/// Collection of internal device parameters, measured by ADSTAT command
#[derive(Debug)]
pub struct InternalDeviceParameters {
    /// Sum of all cells in uV
    pub total_voltage: u32,

    /// Voltage of analog power supply in uV
    pub analog_power: u32,

    /// Voltage of digital power supply in uV
    pub digital_power: u32,

    /// Die temperature in °C as fixed-point number
    /// In case register value overflows 16-bit integer, this value is set to I16F16::MAX (32767.99998)
    pub temperature: I16F16,
}

/// Device specific types
pub trait DeviceTypes: Send + Sync + Sized + 'static {
    /// Argument for the identification of cell groups, which depends on the exact device type.
    type CellSelection: ToCommandBitmap + ToCommandTiming + RegisterLocator<Self> + Copy + Clone + Send + Sync;

    /// Argument for the identification of GPIO groups, which depends on the exact device type.
    type GPIOSelection: ToCommandBitmap + ToCommandTiming + RegisterLocator<Self> + Copy + Clone + Send + Sync;

    /// Argument for register selection. The available registers depend on the device.
    type Register: ToFullCommand + GroupedRegisterIndex + Copy + Clone + Send + Sync;

    /// Available cells and GPIOs
    type Channel: ChannelIndex + Into<ChannelType> + Copy + Clone + Send + Sync;

    /// Number of battery cells supported by the device
    const CELL_COUNT: usize;

    /// Number of GPIO channels
    const GPIO_COUNT: usize;

    /// Defines the first register storing the results of overlap measurement.
    /// None in case overlap test is not supported.
    const OVERLAP_TEST_REG_1: Option<Self::Register>;

    /// Defines the second register storing the results of overlap measurement.
    /// None in case just one cell is ued for overlap test or if test is no supported at all.
    const OVERLAP_TEST_REG_2: Option<Self::Register>;

    /// Status group A register
    const REG_STATUS_A: Self::Register;

    /// Status group b register
    const REG_STATUS_B: Self::Register;

    /// Configuration register A
    const REG_CONF_A: Self::Register;

    /// Configuration register B, None in case device type has no second configuration register
    const REG_CONF_B: Option<Self::Register>;
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
    fn start_conv_cells(
        &mut self,
        mode: ADCMode,
        cells: T::CellSelection,
        dcp: bool,
    ) -> Result<CommandTime, Self::Error>;

    /// Starts GPIOs ADC conversion
    ///
    /// # Arguments
    ///
    /// * `mode`: ADC mode
    /// * `channels`: Measures t:he given GPIO group
    fn start_conv_gpio(&mut self, mode: ADCMode, pins: T::GPIOSelection) -> Result<CommandTime, Self::Error>;

    /// Start the  Overlap Measurements (ADOL command)
    /// Note: This command is not available on LTC6810, as this device only includes one ADC
    ///
    /// # Arguments
    ///
    /// * `mode`: ADC mode
    /// * `dcp`: True if discharge is permitted during conversion
    fn start_overlap_measurement(&mut self, mode: ADCMode, dcp: bool) -> Result<(), Self::Error>;

    /// Starts measuring internal device parameters (ADSTAT command)
    ///
    /// # Arguments
    ///
    /// * `mode`: ADC mode
    /// * `group`: Selection of status parameter to measure
    fn measure_internal_parameters(&mut self, mode: ADCMode, group: StatusGroup) -> Result<CommandTime, Self::Error>;

    /// Reads the values of the given register
    /// Returns one array for each device in daisy chain
    fn read_register(&mut self, register: T::Register) -> Result<[[u16; 3]; L], Self::Error>;

    /// Writes the values of the given register
    /// One 3-bytes array per device in daisy chain
    fn write_register(&mut self, register: T::Register, data: [[u8; 6]; L]) -> Result<(), Self::Error>;

    /// Writes the configuration, one array item per device in daisy chain
    fn write_configuration(&mut self, config: [Configuration; L]) -> Result<(), Self::Error>;

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

    /// Reads and returns the results of the overlap measurement
    ///
    /// Index 0: Result of ADC A of first cell*
    /// Index 1: Result of ADC B of first cell*
    /// Index 2: Result of ADC A of second cell*
    /// Index 3: Result of ADC B of second cell*
    ///
    /// * Number of cells depends on the device type, otherwise 0 value is used
    fn read_overlap_result(&mut self) -> Result<[[u16; 4]; L], Self::Error>;

    /// Reads internal device parameters measured by ATOL command
    /// Returns one array item for each device in daisy chain
    fn read_internal_device_parameters(&mut self) -> Result<Vec<InternalDeviceParameters, L>, Self::Error>;
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
    fn start_conv_cells(
        &mut self,
        mode: ADCMode,
        cells: T::CellSelection,
        dcp: bool,
    ) -> Result<CommandTime, Error<B, CS>> {
        self.cs.set_low().map_err(Error::CSPinError)?;
        let mut command: u16 = 0b0000_0010_0110_0000;

        command |= (mode as u16) << 7;
        command |= cells.to_bitmap();

        if dcp {
            command |= 0b0001_0000;
        }

        self.send_command(command).map_err(Error::TransferError)?;
        self.poll_method.end_command(&mut self.cs).map_err(Error::CSPinError)?;

        Ok(cells.to_conv_command_timing(mode))
    }

    /// See [LTC681XClient::start_conv_gpio](LTC681XClient#tymethod.start_conv_gpio)
    fn start_conv_gpio(&mut self, mode: ADCMode, channels: T::GPIOSelection) -> Result<CommandTime, Error<B, CS>> {
        self.cs.set_low().map_err(Error::CSPinError)?;
        let mut command: u16 = 0b0000_0100_0110_0000;

        command |= (mode as u16) << 7;
        command |= channels.to_bitmap();

        self.send_command(command).map_err(Error::TransferError)?;
        self.poll_method.end_command(&mut self.cs).map_err(Error::CSPinError)?;

        Ok(channels.to_conv_command_timing(mode))
    }

    /// See [LTC681XClient::start_conv_gpio](LTC681XClient#tymethod.start_overlap_measurement)
    fn start_overlap_measurement(&mut self, mode: ADCMode, dcp: bool) -> Result<(), Error<B, CS>> {
        self.cs.set_low().map_err(Error::CSPinError)?;
        let mut command: u16 = 0b0000_0010_0000_0001;

        command |= (mode as u16) << 7;

        if dcp {
            command |= 0b0001_0000;
        }

        self.send_command(command).map_err(Error::TransferError)?;
        self.poll_method.end_command(&mut self.cs).map_err(Error::CSPinError)
    }

    /// See [LTC681XClient::start_conv_gpio](LTC681XClient#tymethod.measure_internal_parameters)
    fn measure_internal_parameters(&mut self, mode: ADCMode, group: StatusGroup) -> Result<CommandTime, Error<B, CS>> {
        self.cs.set_low().map_err(Error::CSPinError)?;
        let mut command: u16 = 0b0000_0100_0110_1000;

        command |= (mode as u16) << 7;
        command |= group.to_bitmap();

        self.send_command(command).map_err(Error::TransferError)?;
        self.poll_method.end_command(&mut self.cs).map_err(Error::CSPinError)?;

        Ok(group.to_conv_command_timing(mode))
    }

    /// See [LTC681XClient::read_cell_voltages](LTC681XClient#tymethod.read_register)
    fn read_register(&mut self, register: T::Register) -> Result<[[u16; 3]; L], Error<B, CS>> {
        self.read_daisy_chain(register.to_read_command())
    }

    /// See [LTC681XClient::read_cell_voltages](LTC681XClient#tymethod.write_register)
    fn write_register(&mut self, register: T::Register, data: [[u8; 6]; L]) -> Result<(), Error<B, CS>> {
        let mut pre_command = match register.to_write_command() {
            Ok(command) => command,
            Err(_) => return Err(Error::ReadOnlyRegister),
        };

        self.cs.set_low().map_err(Error::CSPinError)?;
        self.bus.transfer(&mut pre_command).map_err(Error::TransferError)?;

        for item in &data {
            let mut full_command: [u8; 8] = [0x0; 8];
            full_command[..6].clone_from_slice(item);

            let pec = PEC15::calc(item);
            full_command[6] = pec[0];
            full_command[7] = pec[1];

            self.bus.transfer(&mut full_command).map_err(Error::TransferError)?;
        }

        self.cs.set_high().map_err(Error::CSPinError)?;
        Ok(())
    }

    /// See [LTC681XClient::read_cell_voltages](LTC681XClient#tymethod.write_configuration)
    fn write_configuration(&mut self, config: [Configuration; L]) -> Result<(), Self::Error> {
        let mut register_a = [[0x0u8; 6]; L];
        let mut register_b = [[0x0u8; 6]; L];

        for item in config.iter().enumerate() {
            register_a[item.0] = item.1.register_a;
            register_b[item.0] = item.1.register_b;
        }

        self.write_register(T::REG_CONF_A, register_a)?;

        if let Some(register) = T::REG_CONF_B {
            self.write_register(register, register_b)?;
        }

        Ok(())
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

    /// See [LTC681XClient::read_cell_voltages](LTC681XClient#tymethod.read_overlap_result)
    fn read_overlap_result(&mut self) -> Result<[[u16; 4]; L], Self::Error> {
        let mut data = [[0; 4]; L];

        let register_c = if let Some(register) = T::OVERLAP_TEST_REG_1 {
            self.read_register(register)?
        } else {
            [[0; 3]; L]
        };

        let register_e = if let Some(register) = T::OVERLAP_TEST_REG_2 {
            self.read_register(register)?
        } else {
            [[0; 3]; L]
        };

        for device_index in 0..L {
            data[device_index][0] = register_c[device_index][0];
            data[device_index][1] = register_c[device_index][1];
            data[device_index][2] = register_e[device_index][0];
            data[device_index][3] = register_e[device_index][1];
        }

        Ok(data)
    }

    /// See [LTC681XClient::read_cell_voltages](LTC681XClient#tymethod.read_internal_device_parameters)
    fn read_internal_device_parameters(&mut self) -> Result<Vec<InternalDeviceParameters, L>, Self::Error> {
        let status_a = self.read_register(T::REG_STATUS_A)?;
        let status_b = self.read_register(T::REG_STATUS_B)?;

        let mut parameters = Vec::new();

        for device_index in 0..L {
            let temp_fixed = self.calc_temperature(status_a[device_index][1]);

            let _ = parameters.push(InternalDeviceParameters {
                total_voltage: status_a[device_index][0] as u32 * 30 * 100,
                analog_power: status_a[device_index][2] as u32 * 100,
                digital_power: status_b[device_index][0] as u32 * 100,
                temperature: temp_fixed,
            });
        }

        Ok(parameters)
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
            self.cs.set_high().map_err(Error::CSPinError)?;
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

    /// Calculates the temperature in °C based on raw register value
    fn calc_temperature(&self, value: u16) -> I16F16 {
        if value >= 53744 {
            return I16F16::MAX;
        }

        // Constant of 276 °C, which needs to be subtracted
        const TEMP_SUB: i32 = 20976;

        // Check if temperature is negative
        let temp_i32: i32 = if value >= TEMP_SUB as u16 {
            value as i32 - TEMP_SUB
        } else {
            0 - TEMP_SUB + value as i32
        };

        // Applying factor 100 uV/7.6 mV
        I16F16::from_num(temp_i32) / 76
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
            Error::ReadOnlyRegister => f.debug_struct("ReadOnlyRegister").finish(),
        }
    }
}
