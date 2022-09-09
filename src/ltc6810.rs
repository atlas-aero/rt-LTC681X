//! Device-specific types for [LTC6810](<https://www.analog.com/en/products/ltc6810-1.html>)
use crate::commands::{
    CMD_R_AUX_V_REG_A, CMD_R_AUX_V_REG_B, CMD_R_CELL_V_REG_A, CMD_R_CELL_V_REG_B, CMD_R_CONF_A, CMD_R_STATUS_A,
    CMD_R_STATUS_B, CMD_W_CONF_A,
};
use crate::monitor::{
    ADCMode, ChannelIndex, ChannelType, CommandTime, DeviceTypes, GroupedRegisterIndex, NoPolling, NoWriteCommandError,
    RegisterAddress, RegisterLocator, ToCommandBitmap, ToCommandTiming, ToFullCommand, LTC681X,
};
use core::slice::Iter;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

/// Cell selection for ADC conversion
///
/// See page 63 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/LTC6810-1-6810-2.pdf>)
/// for conversion times
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellSelection {
    /// All cells
    All = 0x0,
    Cell1 = 0x1,
    Cell2 = 0x2,
    Cell3 = 0x3,
    Cell4 = 0x4,
    Cell5 = 0x5,
    Cell6 = 0x6,
}

/// GPIO selection for ADC conversion,
///
/// See page 63 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/LTC6810-1-6810-2.pdf>)
/// for conversion times
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GPIOSelection {
    /// S0, GPIO 1-4 and 2nd Reference
    All = 0x0,
    S0 = 0x1,
    GPIO1 = 0x2,
    GPIO2 = 0x3,
    GPIO3 = 0x4,
    GPIO4 = 0x5,
    SecondReference = 0x6,
}

/// Available registers
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Register {
    CellVoltageA,
    CellVoltageB,
    AuxiliaryA,
    AuxiliaryB,
    StatusA,
    StatusB,
    Configuration,
}

/// All conversion channels
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Channel {
    Cell1,
    Cell2,
    Cell3,
    Cell4,
    Cell5,
    Cell6,
    GPIO1,
    GPIO2,
    GPIO3,
    GPIO4,
    S0,
    SecondReference,
}

/// Device type of LTC6813
pub struct LTC6810 {}

impl DeviceTypes for LTC6810 {
    type CellSelection = CellSelection;
    type GPIOSelection = GPIOSelection;
    type Register = Register;
    type Channel = Channel;

    const CELL_COUNT: usize = 6;
    const GPIO_COUNT: usize = 4;

    const OVERLAP_TEST_REG_1: Option<Self::Register> = None;
    const OVERLAP_TEST_REG_2: Option<Self::Register> = None;

    const REG_STATUS_A: Self::Register = Register::StatusA;
    const REG_STATUS_B: Self::Register = Register::StatusB;

    const REG_CONF_A: Self::Register = Register::Configuration;
    const REG_CONF_B: Option<Self::Register> = None;
}

impl<B, CS, const L: usize> LTC681X<B, CS, NoPolling, LTC6810, L>
where
    B: Transfer<u8>,
    CS: OutputPin,
{
    /// Creates a client instant for LTC6810 variant
    pub fn ltc6810(bus: B, cs: CS) -> Self {
        LTC681X::new(bus, cs)
    }
}

impl ToCommandBitmap for CellSelection {
    fn to_bitmap(&self) -> u16 {
        *self as u16
    }
}

impl ToCommandBitmap for GPIOSelection {
    fn to_bitmap(&self) -> u16 {
        *self as u16
    }
}

impl ToFullCommand for Register {
    /// Returns the precalculated full command
    fn to_read_command(&self) -> [u8; 4] {
        match self {
            Register::CellVoltageA => CMD_R_CELL_V_REG_A,
            Register::CellVoltageB => CMD_R_CELL_V_REG_B,
            Register::AuxiliaryA => CMD_R_AUX_V_REG_A,
            Register::AuxiliaryB => CMD_R_AUX_V_REG_B,
            Register::StatusA => CMD_R_STATUS_A,
            Register::StatusB => CMD_R_STATUS_B,
            Register::Configuration => CMD_R_CONF_A,
        }
    }

    fn to_write_command(&self) -> Result<[u8; 4], NoWriteCommandError> {
        match self {
            Register::Configuration => Ok(CMD_W_CONF_A),
            _ => Err(NoWriteCommandError {}),
        }
    }
}

impl ToCommandTiming for CellSelection {
    fn to_adcv_command_time(&self, mode: ADCMode) -> CommandTime {
        match self {
            CellSelection::All => match mode {
                ADCMode::Fast => CommandTime::new(1106, 1281),
                ADCMode::Normal => CommandTime::new(2328, 3026),
                ADCMode::Filtered => CommandTime::new(201_310, 4423),
                ADCMode::Other => CommandTime::new(12_801, 7215),
            },
            CellSelection::Cell1
            | CellSelection::Cell2
            | CellSelection::Cell3
            | CellSelection::Cell4
            | CellSelection::Cell5
            | CellSelection::Cell6 => match mode {
                ADCMode::Fast => CommandTime::new(200, 229),
                ADCMode::Normal => CommandTime::new(404, 520),
                ADCMode::Filtered => CommandTime::new(33_567, 753),
                ADCMode::Other => CommandTime::new(2149, 1218),
            },
        }
    }
}

impl GroupedRegisterIndex for Register {
    fn to_index(&self) -> usize {
        match self {
            Register::CellVoltageA => 0,
            Register::CellVoltageB => 1,
            Register::AuxiliaryA => 0,
            Register::AuxiliaryB => 1,
            Register::StatusA => 0,
            Register::StatusB => 1,
            Register::Configuration => 0,
        }
    }
}

impl ChannelIndex for Channel {
    fn to_cell_index(&self) -> Option<usize> {
        match self {
            Channel::Cell1 => Some(0),
            Channel::Cell2 => Some(1),
            Channel::Cell3 => Some(2),
            Channel::Cell4 => Some(3),
            Channel::Cell5 => Some(4),
            Channel::Cell6 => Some(5),
            _ => None,
        }
    }

    fn to_gpio_index(&self) -> Option<usize> {
        match self {
            Channel::GPIO1 => Some(0),
            Channel::GPIO2 => Some(1),
            Channel::GPIO3 => Some(2),
            Channel::GPIO4 => Some(3),
            _ => None,
        }
    }
}

impl From<Channel> for ChannelType {
    fn from(channel: Channel) -> Self {
        match channel {
            Channel::GPIO1 => ChannelType::GPIO,
            Channel::GPIO2 => ChannelType::GPIO,
            Channel::GPIO3 => ChannelType::GPIO,
            Channel::GPIO4 => ChannelType::GPIO,
            Channel::SecondReference => ChannelType::Reference,
            _ => ChannelType::Cell,
        }
    }
}

impl RegisterAddress<LTC6810> {
    pub const fn ltc6810(channel: Channel, register: Register, slot: usize) -> Self {
        RegisterAddress {
            channel,
            register,
            slot,
        }
    }
}

const CELL_REGISTER_LOCATIONS: [RegisterAddress<LTC6810>; 6] = [
    RegisterAddress::ltc6810(Channel::Cell1, Register::CellVoltageA, 0),
    RegisterAddress::ltc6810(Channel::Cell2, Register::CellVoltageA, 1),
    RegisterAddress::ltc6810(Channel::Cell3, Register::CellVoltageA, 2),
    RegisterAddress::ltc6810(Channel::Cell4, Register::CellVoltageB, 0),
    RegisterAddress::ltc6810(Channel::Cell5, Register::CellVoltageB, 1),
    RegisterAddress::ltc6810(Channel::Cell6, Register::CellVoltageB, 2),
];

impl RegisterLocator<LTC6810> for CellSelection {
    fn get_locations(&self) -> Iter<'static, RegisterAddress<LTC6810>> {
        match self {
            CellSelection::All => CELL_REGISTER_LOCATIONS.iter(),
            CellSelection::Cell1 => CELL_REGISTER_LOCATIONS[0..1].iter(),
            CellSelection::Cell2 => CELL_REGISTER_LOCATIONS[1..2].iter(),
            CellSelection::Cell3 => CELL_REGISTER_LOCATIONS[2..3].iter(),
            CellSelection::Cell4 => CELL_REGISTER_LOCATIONS[3..4].iter(),
            CellSelection::Cell5 => CELL_REGISTER_LOCATIONS[4..5].iter(),
            CellSelection::Cell6 => CELL_REGISTER_LOCATIONS[5..6].iter(),
        }
    }
}

const GPIO_REGISTER_LOCATIONS: [RegisterAddress<LTC6810>; 6] = [
    RegisterAddress::ltc6810(Channel::S0, Register::AuxiliaryA, 0),
    RegisterAddress::ltc6810(Channel::GPIO1, Register::AuxiliaryA, 1),
    RegisterAddress::ltc6810(Channel::GPIO2, Register::AuxiliaryA, 2),
    RegisterAddress::ltc6810(Channel::GPIO3, Register::AuxiliaryB, 0),
    RegisterAddress::ltc6810(Channel::GPIO4, Register::AuxiliaryB, 1),
    RegisterAddress::ltc6810(Channel::SecondReference, Register::AuxiliaryB, 2),
];

impl RegisterLocator<LTC6810> for GPIOSelection {
    fn get_locations(&self) -> Iter<'static, RegisterAddress<LTC6810>> {
        match self {
            GPIOSelection::All => GPIO_REGISTER_LOCATIONS.iter(),
            GPIOSelection::S0 => GPIO_REGISTER_LOCATIONS[0..1].iter(),
            GPIOSelection::GPIO1 => GPIO_REGISTER_LOCATIONS[1..2].iter(),
            GPIOSelection::GPIO2 => GPIO_REGISTER_LOCATIONS[2..3].iter(),
            GPIOSelection::GPIO3 => GPIO_REGISTER_LOCATIONS[3..4].iter(),
            GPIOSelection::GPIO4 => GPIO_REGISTER_LOCATIONS[4..5].iter(),
            GPIOSelection::SecondReference => GPIO_REGISTER_LOCATIONS[5..6].iter(),
        }
    }
}
