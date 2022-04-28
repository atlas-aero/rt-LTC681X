use crate::commands::*;
use crate::monitor::{DeviceTypes, NoPolling, ToCommandBitmap, ToFullCommand, LTC681X};
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

/// Cell selection for ADC conversion
///
/// See page 62 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/ltc6813-1.pdf>)
/// for conversion times
#[derive(Copy, Clone, PartialEq)]
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
#[derive(Copy, Clone, PartialEq)]
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
#[derive(Copy, Clone, PartialEq)]
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
}

/// Device type of LTC6813
pub struct LTC6813 {}

impl DeviceTypes for LTC6813 {
    type CellSelection = CellSelection;
    type GPIOSelection = GPIOSelection;
    type Register = Register;
}

impl<B, CS, const L: usize> LTC681X<B, CS, NoPolling, LTC6813, L>
where
    B: Transfer<u8>,
    CS: OutputPin,
{
    /// Creates a client instant for LTC6813 variant
    pub fn ltc6813(bus: B, cs: CS) -> Self {
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
    fn to_command(&self) -> [u8; 4] {
        match self {
            Register::CellVoltageA => CMD_CELL_V_REG_A,
            Register::CellVoltageB => CMD_CELL_V_REG_B,
            Register::CellVoltageC => CMD_CELL_V_REG_C,
            Register::CellVoltageD => CMD_CELL_V_REG_D,
            Register::CellVoltageE => CMD_CELL_V_REG_E,
            Register::CellVoltageF => CMD_CELL_V_REG_F,
            Register::AuxiliaryA => CMD_AUX_V_REG_A,
            Register::AuxiliaryB => CMD_AUX_V_REG_B,
            Register::AuxiliaryC => CMD_AUX_V_REG_C,
            Register::AuxiliaryD => CMD_AUX_V_REG_D,
        }
    }
}
