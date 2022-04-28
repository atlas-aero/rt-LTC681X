use crate::commands::{
    CMD_AUX_V_REG_A, CMD_AUX_V_REG_B, CMD_CELL_V_REG_A, CMD_CELL_V_REG_B, CMD_CELL_V_REG_C, CMD_CELL_V_REG_D,
};
use crate::monitor::{DeviceTypes, NoPolling, ToCommandBitmap, ToFullCommand, LTC681X};
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
#[derive(Copy, Clone, PartialEq)]
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
#[derive(Copy, Clone, PartialEq)]
pub enum Register {
    CellVoltageA,
    CellVoltageB,
    CellVoltageC,
    CellVoltageD,
    AuxiliaryA,
    AuxiliaryB,
}

/// Device type of LTC6813
pub struct LTC6811 {}

impl DeviceTypes for LTC6811 {
    type CellSelection = CellSelection;
    type GPIOSelection = GPIOSelection;
    type Register = Register;
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
    fn to_command(&self) -> [u8; 4] {
        match self {
            Register::CellVoltageA => CMD_CELL_V_REG_A,
            Register::CellVoltageB => CMD_CELL_V_REG_B,
            Register::CellVoltageC => CMD_CELL_V_REG_C,
            Register::CellVoltageD => CMD_CELL_V_REG_D,
            Register::AuxiliaryA => CMD_AUX_V_REG_A,
            Register::AuxiliaryB => CMD_AUX_V_REG_B,
        }
    }
}
