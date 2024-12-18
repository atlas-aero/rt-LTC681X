//! Device-specific types for [LTC6813](<https://www.analog.com/en/products/ltc6813-1.html>)
use crate::commands::*;
use crate::monitor::{
    ADCMode, ChannelIndex, ChannelType, CommandTime, DeviceTypes, GroupedRegisterIndex, NoPolling, NoWriteCommandError,
    RegisterAddress, RegisterLocator, ToCommandBitmap, ToCommandTiming, ToFullCommand, LTC681X,
};
use core::slice::Iter;
use embedded_hal::spi::SpiDevice;

/// Cell selection for ADC conversion
///
/// See page 62 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/ltc6813-1.pdf>)
/// for conversion times
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CellSelection {
    /// All cells
    All = 0x0,
    /// Cells 1, 7, 13
    Group1 = 0x1,
    /// Cells 2, 8, 14
    Group2 = 0x2,
    /// Cells 3, 9, 15
    Group3 = 0x3,
    /// Cells 4, 10, 16
    Group4 = 0x4,
    /// Cells 5, 11, 17
    Group5 = 0x5,
    /// cells 6, 12, 18
    Group6 = 0x6,
}

/// GPIO selection for ADC conversion,
///
/// See page 62 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/ltc6813-1.pdf>)
/// for conversion times
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GPIOSelection {
    /// GPIO 1-5, 2nd Reference, GPIO 6-9
    All = 0x0,
    /// GPIO 1 and GPIO 6
    Group1 = 0x1,
    /// GPIO 2 and GPIO 7
    Group2 = 0x2,
    /// GPIO 3 and GPIO 8
    Group3 = 0x3,
    /// GPIO 4 and GPIO 9
    Group4 = 0x4,
    /// GPIO 5
    Group5 = 0x5,
    /// 2nd Reference
    Group6 = 0x6,
}

/// Available registers
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Register {
    CellVoltageA,
    CellVoltageB,
    CellVoltageC,
    CellVoltageD,
    CellVoltageE,
    CellVoltageF,
    AuxiliaryA,
    AuxiliaryB,
    AuxiliaryC,
    AuxiliaryD,
    StatusA,
    StatusB,
    ConfigurationA,
    ConfigurationB,
}

/// All conversion channels
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
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
    Cell13,
    Cell14,
    Cell15,
    Cell16,
    Cell17,
    Cell18,
    GPIO1,
    GPIO2,
    GPIO3,
    GPIO4,
    GPIO5,
    GPIO6,
    GPIO7,
    GPIO8,
    GPIO9,
    SecondReference,
}

/// Device type of LTC6813
#[cfg_attr(test, derive(Debug))]
pub struct LTC6813 {}

impl DeviceTypes for LTC6813 {
    type CellSelection = CellSelection;
    type GPIOSelection = GPIOSelection;
    type Register = Register;
    type Channel = Channel;

    const CELL_COUNT: usize = 18;
    const GPIO_COUNT: usize = 9;

    const OVERLAP_TEST_REG_1: Option<Self::Register> = Some(Register::CellVoltageC);
    const OVERLAP_TEST_REG_2: Option<Self::Register> = Some(Register::CellVoltageE);

    const REG_STATUS_A: Self::Register = Register::StatusA;
    const REG_STATUS_B: Self::Register = Register::StatusB;

    const REG_CONF_A: Self::Register = Register::ConfigurationA;
    const REG_CONF_B: Option<Self::Register> = Some(Register::ConfigurationB);
}

impl<B, const L: usize> LTC681X<B, NoPolling, LTC6813, L>
where
    B: SpiDevice<u8>,
{
    /// Creates a client instant for LTC6813 variant
    pub fn ltc6813(bus: B) -> Self {
        LTC681X::new(bus)
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
            Register::CellVoltageE => CMD_R_CELL_V_REG_E,
            Register::CellVoltageF => CMD_R_CELL_V_REG_F,
            Register::AuxiliaryA => CMD_R_AUX_V_REG_A,
            Register::AuxiliaryB => CMD_R_AUX_V_REG_B,
            Register::AuxiliaryC => CMD_R_AUX_V_REG_C,
            Register::AuxiliaryD => CMD_R_AUX_V_REG_D,
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

impl ToCommandTiming for CellSelection {
    fn to_conv_command_timing(&self, mode: ADCMode) -> CommandTime {
        match self {
            CellSelection::All => match mode {
                ADCMode::Fast => CommandTime::new(1121, 1296),
                ADCMode::Normal => CommandTime::new(2343, 3041),
                ADCMode::Filtered => CommandTime::new(201_325, 4437),
                ADCMode::Other => CommandTime::new(12_816, 7230),
            },
            CellSelection::Group1
            | CellSelection::Group2
            | CellSelection::Group3
            | CellSelection::Group4
            | CellSelection::Group5
            | CellSelection::Group6 => match mode {
                ADCMode::Fast => CommandTime::new(203, 232),
                ADCMode::Normal => CommandTime::new(407, 523),
                ADCMode::Filtered => CommandTime::new(33_570, 756),
                ADCMode::Other => CommandTime::new(2152, 1221),
            },
        }
    }
}

impl ToCommandTiming for GPIOSelection {
    fn to_conv_command_timing(&self, mode: ADCMode) -> CommandTime {
        match self {
            GPIOSelection::All => match mode {
                ADCMode::Fast => CommandTime::new(1825, 2116),
                ADCMode::Normal => CommandTime::new(3862, 5025),
                ADCMode::Filtered => CommandTime::new(335_498, 7353),
                ADCMode::Other => CommandTime::new(21316, 12007),
            },
            GPIOSelection::Group1 | GPIOSelection::Group2 | GPIOSelection::Group3 | GPIOSelection::Group4 => match mode
            {
                ADCMode::Fast => CommandTime::new(380, 439),
                ADCMode::Normal => CommandTime::new(788, 1000),
                ADCMode::Filtered => CommandTime::new(67_100, 1_500),
                ADCMode::Other => CommandTime::new(4_300, 2_4000),
            },
            GPIOSelection::Group5 | GPIOSelection::Group6 => match mode {
                ADCMode::Fast => CommandTime::new(200, 229),
                ADCMode::Normal => CommandTime::new(403, 520),
                ADCMode::Filtered => CommandTime::new(34_000, 753),
                ADCMode::Other => CommandTime::new(2_100, 1_200),
            },
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
            Register::CellVoltageE => 4,
            Register::CellVoltageF => 5,
            Register::AuxiliaryA => 0,
            Register::AuxiliaryB => 1,
            Register::AuxiliaryC => 2,
            Register::AuxiliaryD => 3,
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
            Channel::Cell13 => Some(12),
            Channel::Cell14 => Some(13),
            Channel::Cell15 => Some(14),
            Channel::Cell16 => Some(15),
            Channel::Cell17 => Some(16),
            Channel::Cell18 => Some(17),
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
            Channel::GPIO6 => Some(5),
            Channel::GPIO7 => Some(6),
            Channel::GPIO8 => Some(7),
            Channel::GPIO9 => Some(8),
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
            Channel::GPIO6 => ChannelType::GPIO,
            Channel::GPIO7 => ChannelType::GPIO,
            Channel::GPIO8 => ChannelType::GPIO,
            Channel::GPIO9 => ChannelType::GPIO,
            Channel::SecondReference => ChannelType::Reference,
            _ => ChannelType::Cell,
        }
    }
}

impl RegisterAddress<LTC6813> {
    pub const fn ltc6813(channel: Channel, register: Register, slot: usize) -> Self {
        RegisterAddress {
            channel,
            register,
            slot,
        }
    }
}

const CELL_REGISTER_LOCATIONS: [RegisterAddress<LTC6813>; 18] = [
    RegisterAddress::ltc6813(Channel::Cell1, Register::CellVoltageA, 0),
    RegisterAddress::ltc6813(Channel::Cell7, Register::CellVoltageC, 0),
    RegisterAddress::ltc6813(Channel::Cell13, Register::CellVoltageE, 0),
    RegisterAddress::ltc6813(Channel::Cell2, Register::CellVoltageA, 1),
    RegisterAddress::ltc6813(Channel::Cell8, Register::CellVoltageC, 1),
    RegisterAddress::ltc6813(Channel::Cell14, Register::CellVoltageE, 1),
    RegisterAddress::ltc6813(Channel::Cell3, Register::CellVoltageA, 2),
    RegisterAddress::ltc6813(Channel::Cell9, Register::CellVoltageC, 2),
    RegisterAddress::ltc6813(Channel::Cell15, Register::CellVoltageE, 2),
    RegisterAddress::ltc6813(Channel::Cell4, Register::CellVoltageB, 0),
    RegisterAddress::ltc6813(Channel::Cell10, Register::CellVoltageD, 0),
    RegisterAddress::ltc6813(Channel::Cell16, Register::CellVoltageF, 0),
    RegisterAddress::ltc6813(Channel::Cell5, Register::CellVoltageB, 1),
    RegisterAddress::ltc6813(Channel::Cell11, Register::CellVoltageD, 1),
    RegisterAddress::ltc6813(Channel::Cell17, Register::CellVoltageF, 1),
    RegisterAddress::ltc6813(Channel::Cell6, Register::CellVoltageB, 2),
    RegisterAddress::ltc6813(Channel::Cell12, Register::CellVoltageD, 2),
    RegisterAddress::ltc6813(Channel::Cell18, Register::CellVoltageF, 2),
];

impl RegisterLocator<LTC6813> for CellSelection {
    fn get_locations(&self) -> Iter<'static, RegisterAddress<LTC6813>> {
        match self {
            CellSelection::All => CELL_REGISTER_LOCATIONS.iter(),
            CellSelection::Group1 => CELL_REGISTER_LOCATIONS[0..3].iter(),
            CellSelection::Group2 => CELL_REGISTER_LOCATIONS[3..6].iter(),
            CellSelection::Group3 => CELL_REGISTER_LOCATIONS[6..9].iter(),
            CellSelection::Group4 => CELL_REGISTER_LOCATIONS[9..12].iter(),
            CellSelection::Group5 => CELL_REGISTER_LOCATIONS[12..15].iter(),
            CellSelection::Group6 => CELL_REGISTER_LOCATIONS[15..18].iter(),
        }
    }
}

const GPIO_REGISTER_LOCATIONS: [RegisterAddress<LTC6813>; 10] = [
    RegisterAddress::ltc6813(Channel::GPIO1, Register::AuxiliaryA, 0),
    RegisterAddress::ltc6813(Channel::GPIO6, Register::AuxiliaryC, 0),
    RegisterAddress::ltc6813(Channel::GPIO2, Register::AuxiliaryA, 1),
    RegisterAddress::ltc6813(Channel::GPIO7, Register::AuxiliaryC, 1),
    RegisterAddress::ltc6813(Channel::GPIO3, Register::AuxiliaryA, 2),
    RegisterAddress::ltc6813(Channel::GPIO8, Register::AuxiliaryC, 2),
    RegisterAddress::ltc6813(Channel::GPIO4, Register::AuxiliaryB, 0),
    RegisterAddress::ltc6813(Channel::GPIO9, Register::AuxiliaryD, 0),
    RegisterAddress::ltc6813(Channel::GPIO5, Register::AuxiliaryB, 1),
    RegisterAddress::ltc6813(Channel::SecondReference, Register::AuxiliaryB, 2),
];

impl RegisterLocator<LTC6813> for GPIOSelection {
    fn get_locations(&self) -> Iter<'static, RegisterAddress<LTC6813>> {
        match self {
            GPIOSelection::All => GPIO_REGISTER_LOCATIONS.iter(),
            GPIOSelection::Group1 => GPIO_REGISTER_LOCATIONS[0..2].iter(),
            GPIOSelection::Group2 => GPIO_REGISTER_LOCATIONS[2..4].iter(),
            GPIOSelection::Group3 => GPIO_REGISTER_LOCATIONS[4..6].iter(),
            GPIOSelection::Group4 => GPIO_REGISTER_LOCATIONS[6..8].iter(),
            GPIOSelection::Group5 => GPIO_REGISTER_LOCATIONS[8..9].iter(),
            GPIOSelection::Group6 => GPIO_REGISTER_LOCATIONS[9..10].iter(),
        }
    }
}
