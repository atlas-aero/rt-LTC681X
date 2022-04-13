use crate::mocks::{BusError, BusMockBuilder, MockPin, MockSPIBus, PinError};
use crate::monitor::{ADCMode, CellSelection, Error, LTC681X};
use crate::pec15::PEC15;

#[test]
fn test_start_conv_cells_acc_modes() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0011, 0b0110_0000, 0xf4, 0x6c)
        .expect_command(0b0000_0010, 0b1110_0000, 0x38, 0x06)
        .expect_command(0b0000_0011, 0b1110_0000, 0xb0, 0x4a)
        .expect_command(0b0000_0010, 0b0110_0000, 0x7c, 0x20)
        .to_mock();

    let mut monitor = LTC681X::new(bus, get_cs_no_polling(4));
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false).unwrap();
    monitor.start_conv_cells(ADCMode::Fast, CellSelection::All, false).unwrap();
    monitor.start_conv_cells(ADCMode::Filtered, CellSelection::All, false).unwrap();
    monitor.start_conv_cells(ADCMode::Other, CellSelection::All, false).unwrap();
}

#[test]
fn test_start_conv_cells_cell_groups() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0011, 0b0110_0000, 0xf4, 0x6c)
        .expect_command(0b0000_0011, 0b0110_0001, 0x7F, 0x5E)
        .expect_command(0b0000_0011, 0b0110_0010, 0x69, 0x3A)
        .expect_command(0b0000_0011, 0b0110_0011, 0xE2, 0x8)
        .expect_command(0b0000_0011, 0b0110_0100, 0x45, 0xF2)
        .expect_command(0b0000_0011, 0b0110_0101, 0xCE, 0xC0)
        .expect_command(0b0000_0011, 0b0110_0110, 0xD8, 0xA4)
        .to_mock();

    let mut monitor = LTC681X::new(bus, get_cs_no_polling(7));
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false).unwrap();
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::Group1, false).unwrap();
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::Group2, false).unwrap();
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::Group3, false).unwrap();
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::Group4, false).unwrap();
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::Group5, false).unwrap();
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::Group6, false).unwrap();
}

#[test]
fn test_start_conv_permit_charging() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0011, 0b0110_0000, 0xf4, 0x6c)
        .expect_command(0b0000_0011, 0b0111_0000, 0xAF, 0x42)
        .to_mock();

    let mut monitor = LTC681X::new(bus, get_cs_no_polling(2));
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false).unwrap();
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, true).unwrap();
}

#[test]
fn test_start_conv_cells_sdo_polling() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0011, 0b0110_0000, 0xf4, 0x6c)
        .to_mock();

    let mut monitor = LTC681X::new(bus, cs).enable_sdo_polling();
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false).unwrap();
}

#[test]
fn test_start_conv_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = MockSPIBus::new();
    let mut monitor = LTC681X::new(bus, cs);

    let result = monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false);
    match result.unwrap_err() {
        Error::TransferError(_) => panic!("Unexpected TransferError"),
        Error::CSPinError(_) => {}
    }
}

#[test]
fn test_start_conv_transfer_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor = LTC681X::new(bus, cs);

    let result = monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false);
    match result.unwrap_err() {
        Error::TransferError(_) => {}
        Error::CSPinError(_) => panic!("Unexpected CSPinError"),
    }
}

#[test]
fn test_sdo_polling_ready() {
    let mut cs = MockPin::new();
    cs.expect_set_high().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Ok(&[0xff]));

    let mut monitor = LTC681X::new(bus, cs).enable_sdo_polling();
    assert!(monitor.adc_ready().unwrap());
}

#[test]
fn test_sdo_polling_not_ready() {
    let cs = MockPin::new();

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Ok(&[0x00]));

    let mut monitor = LTC681X::new(bus, cs).enable_sdo_polling();
    assert!(!monitor.adc_ready().unwrap());
}

#[test]
fn test_sdo_polling_transfer_error() {
    let cs = MockPin::new();
    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor = LTC681X::new(bus, cs).enable_sdo_polling();

    match monitor.adc_ready().unwrap_err() {
        Error::TransferError(_) => {}
        Error::CSPinError(_) => panic!("Unexpected CSPinError"),
    }
}

#[test]
fn test_sdo_polling_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_high().times(1).returning(move || Err(PinError::Error1));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Ok(&[0xff]));

    let mut monitor = LTC681X::new(bus, cs).enable_sdo_polling();

    match monitor.adc_ready().unwrap_err() {
        Error::TransferError(_) => panic!("Unexpected TransferError"),
        Error::CSPinError(_) => {}
    }
}

#[test]
fn test_pec15_two_bytes() {
    // STSCTRL command
    assert_eq!([0x8e, 0x4e], PEC15::calc(&[0x0, 0x19]));

    // CLRSCTRL command
    assert_eq!([0x5, 0x7c], PEC15::calc(&[0x0, 0x18]));

    // Cell Voltage register A
    assert_eq!([7, 194], PEC15::calc(&[0x0, 0x4]));
}

#[test]
fn test_pec15_multiple_bytes() {
    assert_eq!([0x37, 0x9e], PEC15::calc(&[0x32, 0x67, 0xF2, 0x1E, 0x5F, 0x24]));
    assert_eq!([0x98, 0x84], PEC15::calc(&[0xCD, 0x62, 0x11, 0x1F, 0x83, 0x24]));
    assert_eq!([0x1B, 0xE6], PEC15::calc(&[0xC9, 0x62, 0x7C, 0x1C, 0x1A, 0x21]));
}

/// Creates a pin mock for no polling method
fn get_cs_no_polling(call_count: usize) -> MockPin {
    let mut cs = MockPin::new();
    cs.expect_set_high().times(call_count).returning(move || Ok(()));
    cs.expect_set_low().times(call_count).returning(move || Ok(()));

    cs
}
