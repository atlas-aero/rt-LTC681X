//! Device-specific types for [LTC6811](<https://www.analog.com/en/products/ltc6811-1.html>)
use crate::commands::{
    CMD_R_AUX_V_REG_A, CMD_R_AUX_V_REG_B, CMD_R_CELL_V_REG_A, CMD_R_CELL_V_REG_B, CMD_R_CELL_V_REG_C,
    CMD_R_CELL_V_REG_D, CMD_R_CONF_A, CMD_R_CONF_B, CMD_R_STATUS_A, CMD_R_STATUS_B, CMD_W_CONF_A, CMD_W_CONF_B,
};
use crate::monitor::{
    ChannelIndex, ChannelType, DeviceTypes, GroupedRegisterIndex, NoPolling, NoWriteCommandError, RegisterAddress,
    RegisterLocator, ToCommandBitmap, ToFullCommand, LTC681X,
};
use core::slice::Iter;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

/// Cell selection for ADC conversion
///
/// See page 61 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/LTC6811-1-6811-2.pdf>)
/// for conversion times
#[derive(Copy, Clone, PartialEq)]
pub enum CellSelection {
    /// All cells
    All = 0x0,
    /// Cells 1 and 7
    Pair1 = 0x1,
    /// Cells 2 and 8
    Pair2 = 0x2,
    /// Cells 3 and 9
    Pair3 = 0x3,
    /// Cells 4 and 10
    Pair4 = 0x4,
    /// Cells 5 and 11
    Pair5 = 0x5,
    /// Cells 6 and 12
    Pair6 = 0x6,
}

/// GPIO selection for ADC conversion,
///
/// See page 61 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/LTC6811-1-6811-2.pdf>)
/// for conversion times
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GPIOSelection {
    /// GPIO 1-5 and 2nd Reference
    All = 0x0,
    GPIO1 = 0x1,
    GPIO2 = 0x2,
    GPIO3 = 0x3,
    GPIO4 = 0x4,
    GPIO5 = 0x5,
    SecondReference = 0x6,
}

/// Available registers
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Register {
    CellVoltageA,
    CellVoltageB,
    CellVoltageC,
    CellVoltageD,
    AuxiliaryA,
    AuxiliaryB,
    StatusA,
    StatusB,
    ConfigurationA,
    ConfigurationB,
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
    Cell7,
    Cell8,
    Cell9,
    Cell10,
    Cell11,
    Cell12,
    GPIO1,
    GPIO2,
    GPIO3,
    GPIO4,
    GPIO5,
    SecondReference,
}

/// Device type of LTC6813
pub struct LTC6811 {}

impl DeviceTypes for LTC6811 {
    type CellSelection = CellSelection;
    type GPIOSelection = GPIOSelection;
    type Register = Register;
    type Channel = Channel;

    const CELL_COUNT: usize = 12;
    const GPIO_COUNT: usize = 5;

    const OVERLAP_TEST_REG_1: Option<Self::Register> = Some(Register::CellVoltageC);
    const OVERLAP_TEST_REG_2: Option<Self::Register> = None;

    const REG_STATUS_A: Self::Register = Register::StatusA;
    const REG_STATUS_B: Self::Register = Register::StatusB;

    const REG_CONF_A: Self::Register = Register::ConfigurationA;
    const REG_CONF_B: Option<Self::Register> = Some(Register::ConfigurationB);
}

impl<B, CS, const L: usize> LTC681X<B, CS, NoPolling, LTC6811, L>
where
    B: Transfer<u8>,
    CS: OutputPin,
{
    /// Creates a client instant for LTC6811 variant
    pub fn ltc6811(bus: B, cs: CS) -> Self {
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
            Register::CellVoltageC => CMD_R_CELL_V_REG_C,
            Register::CellVoltageD => CMD_R_CELL_V_REG_D,
            Register::AuxiliaryA => CMD_R_AUX_V_REG_A,
            Register::AuxiliaryB => CMD_R_AUX_V_REG_B,
            Register::StatusA => CMD_R_STATUS_A,
            Register::StatusB => CMD_R_STATUS_B,
            Register::ConfigurationA => CMD_R_CONF_A,
            Register::ConfigurationB => CMD_R_CONF_B,
        }
    }

    fn to_write_command(&self) -> Result<[u8; 4], NoWriteCommandError> {
        match self {
            Register::ConfigurationA => Ok(CMD_W_CONF_A),
            Register::ConfigurationB => Ok(CMD_W_CONF_B),
            _ => Err(NoWriteCommandError {}),
        }
    }
}

impl GroupedRegisterIndex for Register {
    fn to_index(&self) -> usize {
        match self {
            Register::CellVoltageA => 0,
            Register::CellVoltageB => 1,
            Register::CellVoltageC => 2,
            Register::CellVoltageD => 3,
            Register::AuxiliaryA => 0,
            Register::AuxiliaryB => 1,
            Register::StatusA => 0,
            Register::StatusB => 1,
            Register::ConfigurationA => 0,
            Register::ConfigurationB => 1,
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
            Channel::Cell7 => Some(6),
            Channel::Cell8 => Some(7),
            Channel::Cell9 => Some(8),
            Channel::Cell10 => Some(9),
            Channel::Cell11 => Some(10),
            Channel::Cell12 => Some(11),
            _ => None,
        }
    }

    fn to_gpio_index(&self) -> Option<usize> {
        match self {
            Channel::GPIO1 => Some(0),
            Channel::GPIO2 => Some(1),
            Channel::GPIO3 => Some(2),
            Channel::GPIO4 => Some(3),
            Channel::GPIO5 => Some(4),
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
            Channel::GPIO5 => ChannelType::GPIO,
            Channel::SecondReference => ChannelType::Reference,
            _ => ChannelType::Cell,
        }
    }
}

impl RegisterAddress<LTC6811> {
    pub const fn ltc6811(channel: Channel, register: Register, slot: usize) -> Self {
        RegisterAddress {
            channel,
            register,
            slot,
        }
    }
}

const CELL_REGISTER_LOCATIONS: [RegisterAddress<LTC6811>; 12] = [
    RegisterAddress::ltc6811(Channel::Cell1, Register::CellVoltageA, 0),
    RegisterAddress::ltc6811(Channel::Cell7, Register::CellVoltageC, 0),
    RegisterAddress::ltc6811(Channel::Cell2, Register::CellVoltageA, 1),
    RegisterAddress::ltc6811(Channel::Cell8, Register::CellVoltageC, 1),
    RegisterAddress::ltc6811(Channel::Cell3, Register::CellVoltageA, 2),
    RegisterAddress::ltc6811(Channel::Cell9, Register::CellVoltageC, 2),
    RegisterAddress::ltc6811(Channel::Cell4, Register::CellVoltageB, 0),
    RegisterAddress::ltc6811(Channel::Cell10, Register::CellVoltageD, 0),
    RegisterAddress::ltc6811(Channel::Cell5, Register::CellVoltageB, 1),
    RegisterAddress::ltc6811(Channel::Cell11, Register::CellVoltageD, 1),
    RegisterAddress::ltc6811(Channel::Cell6, Register::CellVoltageB, 2),
    RegisterAddress::ltc6811(Channel::Cell12, Register::CellVoltageD, 2),
];

impl RegisterLocator<LTC6811> for CellSelection {
    fn get_locations(&self) -> Iter<'static, RegisterAddress<LTC6811>> {
        match self {
            CellSelection::All => CELL_REGISTER_LOCATIONS.iter(),
            CellSelection::Pair1 => CELL_REGISTER_LOCATIONS[0..2].iter(),
            CellSelection::Pair2 => CELL_REGISTER_LOCATIONS[2..4].iter(),
            CellSelection::Pair3 => CELL_REGISTER_LOCATIONS[4..6].iter(),
            CellSelection::Pair4 => CELL_REGISTER_LOCATIONS[6..8].iter(),
            CellSelection::Pair5 => CELL_REGISTER_LOCATIONS[8..10].iter(),
            CellSelection::Pair6 => CELL_REGISTER_LOCATIONS[10..12].iter(),
        }
    }
}

const GPIO_REGISTER_LOCATIONS: [RegisterAddress<LTC6811>; 6] = [
    RegisterAddress::ltc6811(Channel::GPIO1, Register::AuxiliaryA, 0),
    RegisterAddress::ltc6811(Channel::GPIO2, Register::AuxiliaryA, 1),
    RegisterAddress::ltc6811(Channel::GPIO3, Register::AuxiliaryA, 2),
    RegisterAddress::ltc6811(Channel::GPIO4, Register::AuxiliaryB, 0),
    RegisterAddress::ltc6811(Channel::GPIO5, Register::AuxiliaryB, 1),
    RegisterAddress::ltc6811(Channel::SecondReference, Register::AuxiliaryB, 2),
];

impl RegisterLocator<LTC6811> for GPIOSelection {
    fn get_locations(&self) -> Iter<'static, RegisterAddress<LTC6811>> {
        match self {
            GPIOSelection::All => GPIO_REGISTER_LOCATIONS.iter(),
            GPIOSelection::GPIO1 => GPIO_REGISTER_LOCATIONS[0..1].iter(),
            GPIOSelection::GPIO2 => GPIO_REGISTER_LOCATIONS[1..2].iter(),
            GPIOSelection::GPIO3 => GPIO_REGISTER_LOCATIONS[2..3].iter(),
            GPIOSelection::GPIO4 => GPIO_REGISTER_LOCATIONS[3..4].iter(),
            GPIOSelection::GPIO5 => GPIO_REGISTER_LOCATIONS[4..5].iter(),
            GPIOSelection::SecondReference => GPIO_REGISTER_LOCATIONS[5..6].iter(),
        }
    }
}
