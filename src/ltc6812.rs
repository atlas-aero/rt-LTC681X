use crate::commands::{
    CMD_AUX_V_REG_A, CMD_AUX_V_REG_B, CMD_AUX_V_REG_C, CMD_AUX_V_REG_D, CMD_CELL_V_REG_A, CMD_CELL_V_REG_B,
    CMD_CELL_V_REG_C, CMD_CELL_V_REG_D, CMD_CELL_V_REG_E,
};
use crate::monitor::{NoPolling, ToCommandBitmap, ToFullCommand, LTC681X};
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

/// Cell selection for ADC conversion
///
/// See page 61 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/ltc6812-1.pdf>)
/// for conversion times
#[derive(Copy, Clone, PartialEq)]
pub enum CellSelection {
    /// All cells
    All = 0x0,
    /// Cells 1, 6, 11
    Group1 = 0x1,
    /// Cells 2, 7, 12
    Group2 = 0x2,
    /// Cells 3, 8, 13
    Group3 = 0x3,
    /// Cells 4, 9, 14
    Group4 = 0x4,
    /// Cells 5, 10, 15
    Group5 = 0x5,
}

/// GPIO selection for ADC conversion,
///
/// See page 61 of [datasheet](<https://www.analog.com/media/en/technical-documentation/data-sheets/ltc6812-1.pdf>)
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

/// Cell voltage registers
#[derive(Copy, Clone, PartialEq)]
pub enum CellVoltageRegister {
    RegisterA,
    RegisterB,
    RegisterC,
    RegisterD,
    RegisterE,
}

/// Auxiliary registers
#[derive(Copy, Clone, PartialEq)]
pub enum AuxiliaryRegister {
    RegisterA,
    RegisterB,
    RegisterC,
    RegisterD,
}

impl<B, CS, const L: usize>
    LTC681X<B, CS, NoPolling, CellSelection, GPIOSelection, CellVoltageRegister, AuxiliaryRegister, L>
where
    B: Transfer<u8>,
    CS: OutputPin,
{
    /// Creates a client instant for LTC6812 variant
    pub fn ltc6812(bus: B, cs: CS) -> Self {
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

impl ToFullCommand for CellVoltageRegister {
    /// Returns the precalculated full command
    fn to_command(&self) -> [u8; 4] {
        match self {
            CellVoltageRegister::RegisterA => CMD_CELL_V_REG_A,
            CellVoltageRegister::RegisterB => CMD_CELL_V_REG_B,
            CellVoltageRegister::RegisterC => CMD_CELL_V_REG_C,
            CellVoltageRegister::RegisterD => CMD_CELL_V_REG_D,
            CellVoltageRegister::RegisterE => CMD_CELL_V_REG_E,
        }
    }
}

impl ToFullCommand for AuxiliaryRegister {
    /// Returns the precalculated full command
    fn to_command(&self) -> [u8; 4] {
        match self {
            AuxiliaryRegister::RegisterA => CMD_AUX_V_REG_A,
            AuxiliaryRegister::RegisterB => CMD_AUX_V_REG_B,
            AuxiliaryRegister::RegisterC => CMD_AUX_V_REG_C,
            AuxiliaryRegister::RegisterD => CMD_AUX_V_REG_D,
        }
    }
}
