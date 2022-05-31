//! Tests for generic, device type independent, logic
use crate::ltc6813::{CellSelection, Channel, GPIOSelection, Register};
use crate::mocks::{BusError, BusMockBuilder, MockPin, MockSPIBus, PinError};
use crate::monitor::{ADCMode, Error, LTC681XClient, PollClient, StatusGroup, LTC681X};

#[test]
fn test_start_conv_cells_acc_modes() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0011, 0b0110_0000, 0xf4, 0x6c)
        .expect_command(0b0000_0010, 0b1110_0000, 0x38, 0x06)
        .expect_command(0b0000_0011, 0b1110_0000, 0xb0, 0x4a)
        .expect_command(0b0000_0010, 0b0110_0000, 0x7c, 0x20)
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(4));
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
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(7));
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
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(2));
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false).unwrap();
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, true).unwrap();
}

#[test]
fn test_start_conv_cells_sdo_polling() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0011, 0b0110_0000, 0xf4, 0x6c)
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false).unwrap();
}

#[test]
fn test_start_conv_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = MockSPIBus::new();
    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false);
    match result.unwrap_err() {
        Error::CSPinError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_start_conv_transfer_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false);
    match result.unwrap_err() {
        Error::TransferError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_start_conv_gpio_acc_modes() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0101, 0b0110_0000, 0xD3, 0xA0)
        .expect_command(0b0000_0100, 0b1110_0000, 0x1F, 0xCA)
        .expect_command(0b0000_0101, 0b1110_0000, 0x97, 0x86)
        .expect_command(0b0000_0100, 0b0110_0000, 0x5B, 0xEC)
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(4));
    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All).unwrap();
    monitor.start_conv_gpio(ADCMode::Fast, GPIOSelection::All).unwrap();
    monitor.start_conv_gpio(ADCMode::Filtered, GPIOSelection::All).unwrap();
    monitor.start_conv_gpio(ADCMode::Other, GPIOSelection::All).unwrap();
}

#[test]
fn test_start_conv_gpio_cell_groups() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0101, 0b0110_0000, 0xD3, 0xA0)
        .expect_command(0b0000_0101, 0b0110_0001, 0x58, 0x92)
        .expect_command(0b0000_0101, 0b0110_0010, 0x4E, 0xF6)
        .expect_command(0b0000_0101, 0b0110_0011, 0xC5, 0xC4)
        .expect_command(0b0000_0101, 0b0110_0100, 0x62, 0x3E)
        .expect_command(0b0000_0101, 0b0110_0101, 0xE9, 0xC)
        .expect_command(0b0000_0101, 0b0110_0110, 0xFF, 0x68)
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(7));
    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All).unwrap();
    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group1).unwrap();
    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group2).unwrap();
    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group3).unwrap();
    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group4).unwrap();
    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group5).unwrap();
    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group6).unwrap();
}

#[test]
fn test_start_conv_gpio_sdo_polling() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0101, 0b0110_0000, 0xD3, 0xA0)
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();
    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All).unwrap();
}

#[test]
fn test_start_conv_gpio_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = MockSPIBus::new();
    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All);
    match result.unwrap_err() {
        Error::CSPinError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_start_conv_gpio_transfer_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All);
    match result.unwrap_err() {
        Error::TransferError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_start_overlap_measurement_acc_modes() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0011, 0b0000_0001, 0x2E, 0x88)
        .expect_command(0b0000_0010, 0b1000_0001, 0xE2, 0xE2)
        .expect_command(0b0000_0011, 0b1000_0001, 0x6A, 0xAE)
        .expect_command(0b0000_0010, 0b0000_0001, 0xA6, 0xC4)
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(4));
    monitor.start_overlap_measurement(ADCMode::Normal, false).unwrap();
    monitor.start_overlap_measurement(ADCMode::Fast, false).unwrap();
    monitor.start_overlap_measurement(ADCMode::Filtered, false).unwrap();
    monitor.start_overlap_measurement(ADCMode::Other, false).unwrap();
}

#[test]
fn test_start_overlap_measurement_dcp() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0011, 0b0001_0001, 0x75, 0xA6)
        .expect_command(0b0000_0011, 0b0000_0001, 0x2E, 0x88)
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(2));
    monitor.start_overlap_measurement(ADCMode::Normal, true).unwrap();
    monitor.start_overlap_measurement(ADCMode::Normal, false).unwrap();
}

#[test]
fn test_start_overlap_measurement_sdo_polling() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0011, 0b0000_0001, 0x2E, 0x88)
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();
    monitor.start_overlap_measurement(ADCMode::Normal, false).unwrap();
}

#[test]
fn test_start_overlap_measurement_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = MockSPIBus::new();
    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.start_overlap_measurement(ADCMode::Normal, false);
    match result.unwrap_err() {
        Error::CSPinError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_start_overlap_measurement_transfer_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.start_overlap_measurement(ADCMode::Normal, false);
    match result.unwrap_err() {
        Error::TransferError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_measure_internal_parameters_acc_modes() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0011, 0b0110_1000, 0x1C, 0x62)
        .expect_command(0b0000_0010, 0b1110_1000, 0xD0, 0x8)
        .expect_command(0b0000_0011, 0b1110_1000, 0x58, 0x44)
        .expect_command(0b0000_0010, 0b0110_1000, 0x94, 0x2E)
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(4));
    monitor.measure_internal_parameters(ADCMode::Normal, StatusGroup::All).unwrap();
    monitor.measure_internal_parameters(ADCMode::Fast, StatusGroup::All).unwrap();
    monitor
        .measure_internal_parameters(ADCMode::Filtered, StatusGroup::All)
        .unwrap();
    monitor.measure_internal_parameters(ADCMode::Other, StatusGroup::All).unwrap();
}

#[test]
fn test_measure_internal_parameters_status_groups() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0011, 0b0110_1000, 0x1C, 0x62)
        .expect_command(0b0000_0011, 0b0110_1001, 0x97, 0x50)
        .expect_command(0b0000_0011, 0b0110_1010, 0x81, 0x34)
        .expect_command(0b0000_0011, 0b0110_1011, 0xA, 0x6)
        .expect_command(0b0000_0011, 0b0110_1100, 0xAD, 0xFC)
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(5));
    monitor.measure_internal_parameters(ADCMode::Normal, StatusGroup::All).unwrap();
    monitor
        .measure_internal_parameters(ADCMode::Normal, StatusGroup::CellSum)
        .unwrap();
    monitor
        .measure_internal_parameters(ADCMode::Normal, StatusGroup::Temperature)
        .unwrap();
    monitor
        .measure_internal_parameters(ADCMode::Normal, StatusGroup::AnalogVoltage)
        .unwrap();
    monitor
        .measure_internal_parameters(ADCMode::Normal, StatusGroup::DigitalVoltage)
        .unwrap();
}

#[test]
fn test_measure_internal_parameters_sdo_polling() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0011, 0b0110_1000, 0x1C, 0x62)
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();
    monitor.measure_internal_parameters(ADCMode::Normal, StatusGroup::All).unwrap();
}

#[test]
fn test_measure_internal_parameters_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = MockSPIBus::new();
    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.measure_internal_parameters(ADCMode::Normal, StatusGroup::All);
    match result.unwrap_err() {
        Error::CSPinError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_measure_internal_parameters_transfer_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.measure_internal_parameters(ADCMode::Normal, StatusGroup::All);
    match result.unwrap_err() {
        Error::TransferError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_sdo_polling_ready() {
    let mut cs = MockPin::new();
    cs.expect_set_high().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Ok(&[0xff]));

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();
    assert!(monitor.adc_ready().unwrap());
}

#[test]
fn test_sdo_polling_not_ready() {
    let cs = MockPin::new();

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Ok(&[0x00]));

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();
    assert!(!monitor.adc_ready().unwrap());
}

#[test]
fn test_sdo_polling_transfer_error() {
    let cs = MockPin::new();
    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();

    match monitor.adc_ready().unwrap_err() {
        Error::TransferError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_sdo_polling_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_high().times(1).returning(move || Err(PinError::Error1));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Ok(&[0xff]));

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();

    match monitor.adc_ready().unwrap_err() {
        Error::CSPinError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_cell_voltages_register_a() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_0100, 0x07, 0xC2)
        .expect_register_read(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::CellVoltageA).unwrap();
    assert_eq!(24979, result[0][0]);
    assert_eq!(7867, result[0][1]);
    assert_eq!(8878, result[0][2]);
}

#[test]
fn test_read_cell_voltages_register_b() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_0110, 0x9A, 0x94)
        .expect_register_read(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::CellVoltageB).unwrap();
    assert_eq!(26333, result[0][0]);
    assert_eq!(7538, result[0][1]);
    assert_eq!(7330, result[0][2]);
}

#[test]
fn test_read_cell_voltages_register_c() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1000, 0x5E, 0x52)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::CellVoltageC).unwrap();
    assert_eq!(25441, result[0][0]);
    assert_eq!(7869, result[0][1]);
    assert_eq!(8932, result[0][2]);
}

#[test]
fn test_read_cell_voltages_register_d() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1010, 0xC3, 0x4)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::CellVoltageD).unwrap();
    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);
}

#[test]
fn test_read_cell_voltages_register_e() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1001, 0xD5, 0x60)
        .expect_register_read(&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::CellVoltageE).unwrap();
    assert_eq!(25822, result[0][0]);
    assert_eq!(8591, result[0][1]);
    assert_eq!(8586, result[0][2]);
}

#[test]
fn test_read_cell_voltages_register_f() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1011, 0x48, 0x36)
        .expect_register_read(&[0x00, 0x63, 0x2F, 0x1F, 0x8B, 0x1F, 0xC1, 0x68])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::CellVoltageF).unwrap();
    assert_eq!(25344, result[0][0]);
    assert_eq!(7983, result[0][1]);
    assert_eq!(8075, result[0][2]);
}

#[test]
fn test_read_cell_voltages_multiple_devices() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1010, 0xC3, 0x4)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        .expect_register_read(&[0x53, 0x64, 0x76, 0x1E, 0xB9, 0x1E, 0x1B, 0xC6])
        .expect_register_read(&[0xA2, 0x62, 0x05, 0x1F, 0xC9, 0x20, 0xEE, 0x94])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 3> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::CellVoltageD).unwrap();

    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);

    assert_eq!(25683, result[1][0]);
    assert_eq!(7798, result[1][1]);
    assert_eq!(7865, result[1][2]);

    assert_eq!(25250, result[2][0]);
    assert_eq!(7941, result[2][1]);
    assert_eq!(8393, result[2][2]);
}

#[test]
fn test_read_cell_voltages_pec_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1011, 0x48, 0x36)
        .expect_register_read(&[0x2A, 0x63, 0x8E, 0x1E, 0xEC, 0x1F, 0x11, 0x0D])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_register(Register::CellVoltageF);
    match result.unwrap_err() {
        Error::ChecksumMismatch => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_cell_voltages_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = MockSPIBus::new();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_register(Register::CellVoltageF);
    match result.unwrap_err() {
        Error::CSPinError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_cell_voltages_transfer_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_register(Register::CellVoltageF);
    match result.unwrap_err() {
        Error::TransferError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_register_aux_a() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1100, 0xEF, 0xCC)
        .expect_register_read(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::AuxiliaryA).unwrap();
    assert_eq!(24979, result[0][0]);
    assert_eq!(7867, result[0][1]);
    assert_eq!(8878, result[0][2]);
}

#[test]
fn test_read_register_aux_b() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1110, 0x72, 0x9A)
        .expect_register_read(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::AuxiliaryB).unwrap();
    assert_eq!(26333, result[0][0]);
    assert_eq!(7538, result[0][1]);
    assert_eq!(7330, result[0][2]);
}

#[test]
fn test_read_register_aux_c() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1101, 0x64, 0xFE)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::AuxiliaryC).unwrap();
    assert_eq!(25441, result[0][0]);
    assert_eq!(7869, result[0][1]);
    assert_eq!(8932, result[0][2]);
}

#[test]
fn test_read_register_aux_d() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1111, 0xF9, 0xA8)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::AuxiliaryD).unwrap();
    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);
}

#[test]
fn test_read_register_status_a() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0001_0000, 0xED, 0x72)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::StatusA).unwrap();
    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);
}

#[test]
fn test_read_register_status_b() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0001_0010, 0x70, 0x24)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::StatusB).unwrap();
    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);
}

#[test]
fn test_read_register_multiple_devices() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1111, 0xF9, 0xA8)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        .expect_register_read(&[0x53, 0x64, 0x76, 0x1E, 0xB9, 0x1E, 0x1B, 0xC6])
        .expect_register_read(&[0xA2, 0x62, 0x05, 0x1F, 0xC9, 0x20, 0xEE, 0x94])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 3> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_register(Register::AuxiliaryD).unwrap();

    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);

    assert_eq!(25683, result[1][0]);
    assert_eq!(7798, result[1][1]);
    assert_eq!(7865, result[1][2]);

    assert_eq!(25250, result[2][0]);
    assert_eq!(7941, result[2][1]);
    assert_eq!(8393, result[2][2]);
}

#[test]
fn test_read_register_pec_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1111, 0xF9, 0xA8)
        .expect_register_read(&[0x2A, 0x63, 0x8E, 0x1E, 0xEC, 0x1F, 0x11, 0x0D])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_register(Register::AuxiliaryD);
    match result.unwrap_err() {
        Error::ChecksumMismatch => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_register_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = MockSPIBus::new();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_register(Register::AuxiliaryD);
    match result.unwrap_err() {
        Error::CSPinError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_register_transfer_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_register(Register::AuxiliaryD);
    match result.unwrap_err() {
        Error::TransferError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_voltages_cell_group_1() {
    let bus = BusMockBuilder::new()
        // Register A
        .expect_command(0b0000_0000, 0b0000_0100, 0x07, 0xC2)
        .expect_register_read(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C])
        // Register C
        .expect_command(0b0000_0000, 0b0000_1000, 0x5E, 0x52)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        // Register E
        .expect_command(0b0000_0000, 0b0000_1001, 0xD5, 0x60)
        .expect_register_read(&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(3));

    let result = monitor.read_voltages(CellSelection::Group1).unwrap();
    assert_eq!(3, result[0].len());

    assert_eq!(Channel::Cell1, result[0][0].channel);
    assert_eq!(24979, result[0][0].voltage);

    assert_eq!(Channel::Cell7, result[0][1].channel);
    assert_eq!(25441, result[0][1].voltage);

    assert_eq!(Channel::Cell13, result[0][2].channel);
    assert_eq!(25822, result[0][2].voltage);
}

#[test]
fn test_read_voltages_cell_group_2() {
    let bus = BusMockBuilder::new()
        // Register A
        .expect_command(0b0000_0000, 0b0000_0100, 0x07, 0xC2)
        .expect_register_read(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C])
        // Register C
        .expect_command(0b0000_0000, 0b0000_1000, 0x5E, 0x52)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        // Register E
        .expect_command(0b0000_0000, 0b0000_1001, 0xD5, 0x60)
        .expect_register_read(&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(3));

    let result = monitor.read_voltages(CellSelection::Group2).unwrap();
    assert_eq!(3, result[0].len());

    assert_eq!(Channel::Cell2, result[0][0].channel);
    assert_eq!(7867, result[0][0].voltage);

    assert_eq!(Channel::Cell8, result[0][1].channel);
    assert_eq!(7869, result[0][1].voltage);

    assert_eq!(Channel::Cell14, result[0][2].channel);
    assert_eq!(8591, result[0][2].voltage);
}

#[test]
fn test_read_voltages_cell_group_3() {
    let bus = BusMockBuilder::new()
        // Register A
        .expect_command(0b0000_0000, 0b0000_0100, 0x07, 0xC2)
        .expect_register_read(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C])
        // Register C
        .expect_command(0b0000_0000, 0b0000_1000, 0x5E, 0x52)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        // Register E
        .expect_command(0b0000_0000, 0b0000_1001, 0xD5, 0x60)
        .expect_register_read(&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(3));

    let result = monitor.read_voltages(CellSelection::Group3).unwrap();
    assert_eq!(3, result[0].len());

    assert_eq!(Channel::Cell3, result[0][0].channel);
    assert_eq!(8878, result[0][0].voltage);

    assert_eq!(Channel::Cell9, result[0][1].channel);
    assert_eq!(8932, result[0][1].voltage);

    assert_eq!(Channel::Cell15, result[0][2].channel);
    assert_eq!(8586, result[0][2].voltage);
}

#[test]
fn test_read_voltages_cell_group_4() {
    let bus = BusMockBuilder::new()
        // Register B
        .expect_command(0b0000_0000, 0b0000_0110, 0x9A, 0x94)
        .expect_register_read(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94])
        // Register D
        .expect_command(0b0000_0000, 0b0000_1010, 0xC3, 0x4)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        // Register F
        .expect_command(0b0000_0000, 0b0000_1011, 0x48, 0x36)
        .expect_register_read(&[0x00, 0x63, 0x2F, 0x1F, 0x8B, 0x1F, 0xC1, 0x68])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(3));

    let result = monitor.read_voltages(CellSelection::Group4).unwrap();
    assert_eq!(3, result[0].len());

    assert_eq!(Channel::Cell4, result[0][0].channel);
    assert_eq!(26333, result[0][0].voltage);

    assert_eq!(Channel::Cell10, result[0][1].channel);
    assert_eq!(24970, result[0][1].voltage);

    assert_eq!(Channel::Cell16, result[0][2].channel);
    assert_eq!(25344, result[0][2].voltage);
}

#[test]
fn test_read_voltages_cell_group_5() {
    let bus = BusMockBuilder::new()
        // Register B
        .expect_command(0b0000_0000, 0b0000_0110, 0x9A, 0x94)
        .expect_register_read(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94])
        // Register D
        .expect_command(0b0000_0000, 0b0000_1010, 0xC3, 0x4)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        // Register F
        .expect_command(0b0000_0000, 0b0000_1011, 0x48, 0x36)
        .expect_register_read(&[0x00, 0x63, 0x2F, 0x1F, 0x8B, 0x1F, 0xC1, 0x68])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(3));

    let result = monitor.read_voltages(CellSelection::Group5).unwrap();
    assert_eq!(3, result[0].len());

    assert_eq!(Channel::Cell5, result[0][0].channel);
    assert_eq!(7538, result[0][0].voltage);

    assert_eq!(Channel::Cell11, result[0][1].channel);
    assert_eq!(8033, result[0][1].voltage);

    assert_eq!(Channel::Cell17, result[0][2].channel);
    assert_eq!(7983, result[0][2].voltage);
}

#[test]
fn test_read_voltages_cell_group_6() {
    let bus = BusMockBuilder::new()
        // Register B
        .expect_command(0b0000_0000, 0b0000_0110, 0x9A, 0x94)
        .expect_register_read(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94])
        // Register D
        .expect_command(0b0000_0000, 0b0000_1010, 0xC3, 0x4)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        // Register F
        .expect_command(0b0000_0000, 0b0000_1011, 0x48, 0x36)
        .expect_register_read(&[0x00, 0x63, 0x2F, 0x1F, 0x8B, 0x1F, 0xC1, 0x68])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(3));

    let result = monitor.read_voltages(CellSelection::Group6).unwrap();
    assert_eq!(3, result[0].len());

    assert_eq!(Channel::Cell6, result[0][0].channel);
    assert_eq!(7330, result[0][0].voltage);

    assert_eq!(Channel::Cell12, result[0][1].channel);
    assert_eq!(8655, result[0][1].voltage);

    assert_eq!(Channel::Cell18, result[0][2].channel);
    assert_eq!(8075, result[0][2].voltage);
}

#[test]
fn test_read_voltages_cell_all() {
    let bus = BusMockBuilder::new()
        // Register A
        .expect_command(0b0000_0000, 0b0000_0100, 0x07, 0xC2)
        .expect_register_read(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C])
        // Register C
        .expect_command(0b0000_0000, 0b0000_1000, 0x5E, 0x52)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        // Register E
        .expect_command(0b0000_0000, 0b0000_1001, 0xD5, 0x60)
        .expect_register_read(&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA])
        // Register B
        .expect_command(0b0000_0000, 0b0000_0110, 0x9A, 0x94)
        .expect_register_read(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94])
        // Register D
        .expect_command(0b0000_0000, 0b0000_1010, 0xC3, 0x4)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        // Register F
        .expect_command(0b0000_0000, 0b0000_1011, 0x48, 0x36)
        .expect_register_read(&[0x00, 0x63, 0x2F, 0x1F, 0x8B, 0x1F, 0xC1, 0x68])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(6));

    let mut result = monitor.read_voltages(CellSelection::All).unwrap();
    assert_eq!(18, result[0].len());
    result[0].sort_by_key(|element| element.channel);

    assert_eq!(Channel::Cell1, result[0][0].channel);
    assert_eq!(24979, result[0][0].voltage);

    assert_eq!(Channel::Cell7, result[0][6].channel);
    assert_eq!(25441, result[0][6].voltage);

    assert_eq!(Channel::Cell13, result[0][12].channel);
    assert_eq!(25822, result[0][12].voltage);

    assert_eq!(Channel::Cell2, result[0][1].channel);
    assert_eq!(7867, result[0][1].voltage);

    assert_eq!(Channel::Cell8, result[0][7].channel);
    assert_eq!(7869, result[0][7].voltage);

    assert_eq!(Channel::Cell14, result[0][13].channel);
    assert_eq!(8591, result[0][13].voltage);

    assert_eq!(Channel::Cell3, result[0][2].channel);
    assert_eq!(8878, result[0][2].voltage);

    assert_eq!(Channel::Cell9, result[0][8].channel);
    assert_eq!(8932, result[0][8].voltage);

    assert_eq!(Channel::Cell15, result[0][14].channel);
    assert_eq!(8586, result[0][14].voltage);

    assert_eq!(Channel::Cell4, result[0][3].channel);
    assert_eq!(26333, result[0][3].voltage);

    assert_eq!(Channel::Cell10, result[0][9].channel);
    assert_eq!(24970, result[0][9].voltage);

    assert_eq!(Channel::Cell16, result[0][15].channel);
    assert_eq!(25344, result[0][15].voltage);

    assert_eq!(Channel::Cell5, result[0][4].channel);
    assert_eq!(7538, result[0][4].voltage);

    assert_eq!(Channel::Cell11, result[0][10].channel);
    assert_eq!(8033, result[0][10].voltage);

    assert_eq!(Channel::Cell17, result[0][16].channel);
    assert_eq!(7983, result[0][16].voltage);

    assert_eq!(Channel::Cell6, result[0][5].channel);
    assert_eq!(7330, result[0][5].voltage);

    assert_eq!(Channel::Cell12, result[0][11].channel);
    assert_eq!(8655, result[0][11].voltage);

    assert_eq!(Channel::Cell18, result[0][17].channel);
    assert_eq!(8075, result[0][17].voltage);
}

#[test]
fn test_read_voltages_cell_multiple_devices() {
    let bus = BusMockBuilder::new()
        // Register A
        .expect_command(0b0000_0000, 0b0000_0100, 0x07, 0xC2)
        .expect_register_read(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C])
        .expect_register_read(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94])
        // Register C
        .expect_command(0b0000_0000, 0b0000_1000, 0x5E, 0x52)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        // Register E
        .expect_command(0b0000_0000, 0b0000_1001, 0xD5, 0x60)
        .expect_register_read(&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA])
        .expect_register_read(&[0x00, 0x63, 0x2F, 0x1F, 0x8B, 0x1F, 0xC1, 0x68])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 2> = LTC681X::ltc6813(bus, get_cs_no_polling(3));

    let result = monitor.read_voltages(CellSelection::Group1).unwrap();
    assert_eq!(3, result[0].len());
    assert_eq!(3, result[1].len());

    assert_eq!(Channel::Cell1, result[0][0].channel);
    assert_eq!(Channel::Cell1, result[1][0].channel);
    assert_eq!(24979, result[0][0].voltage);
    assert_eq!(26333, result[1][0].voltage);

    assert_eq!(Channel::Cell7, result[0][1].channel);
    assert_eq!(Channel::Cell7, result[1][1].channel);
    assert_eq!(25441, result[0][1].voltage);
    assert_eq!(24970, result[1][1].voltage);

    assert_eq!(Channel::Cell13, result[0][2].channel);
    assert_eq!(Channel::Cell13, result[1][2].channel);
    assert_eq!(25822, result[0][2].voltage);
    assert_eq!(25344, result[1][2].voltage);
}

#[test]
fn test_read_voltages_gpio_group_1() {
    let bus = BusMockBuilder::new()
        // Register A
        .expect_command(0b0000_0000, 0b0000_1100, 0xEF, 0xCC)
        .expect_register_read(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C])
        // Register C
        .expect_command(0b0000_0000, 0b0000_1101, 0x64, 0xFE)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(2));

    let result = monitor.read_voltages(GPIOSelection::Group1).unwrap();
    assert_eq!(2, result[0].len());

    assert_eq!(Channel::GPIO1, result[0][0].channel);
    assert_eq!(24979, result[0][0].voltage);

    assert_eq!(Channel::GPIO6, result[0][1].channel);
    assert_eq!(25441, result[0][1].voltage);
}

#[test]
fn test_read_voltages_gpio_group_2() {
    let bus = BusMockBuilder::new()
        // Register A
        .expect_command(0b0000_0000, 0b0000_1100, 0xEF, 0xCC)
        .expect_register_read(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C])
        // Register C
        .expect_command(0b0000_0000, 0b0000_1101, 0x64, 0xFE)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(2));

    let result = monitor.read_voltages(GPIOSelection::Group2).unwrap();
    assert_eq!(2, result[0].len());

    assert_eq!(Channel::GPIO2, result[0][0].channel);
    assert_eq!(7867, result[0][0].voltage);

    assert_eq!(Channel::GPIO7, result[0][1].channel);
    assert_eq!(7869, result[0][1].voltage);
}

#[test]
fn test_read_voltages_gpio_group_3() {
    let bus = BusMockBuilder::new()
        // Register A
        .expect_command(0b0000_0000, 0b0000_1100, 0xEF, 0xCC)
        .expect_register_read(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C])
        // Register C
        .expect_command(0b0000_0000, 0b0000_1101, 0x64, 0xFE)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(2));

    let result = monitor.read_voltages(GPIOSelection::Group3).unwrap();
    assert_eq!(2, result[0].len());

    assert_eq!(Channel::GPIO3, result[0][0].channel);
    assert_eq!(8878, result[0][0].voltage);

    assert_eq!(Channel::GPIO8, result[0][1].channel);
    assert_eq!(8932, result[0][1].voltage);
}

#[test]
fn test_read_voltages_gpio_group_4() {
    let bus = BusMockBuilder::new()
        // Register D
        .expect_command(0b0000_0000, 0b0000_1110, 0x72, 0x9A)
        .expect_register_read(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94])
        // Register B
        .expect_command(0b0000_0000, 0b0000_1111, 0xF9, 0xA8)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(2));

    let result = monitor.read_voltages(GPIOSelection::Group4).unwrap();
    assert_eq!(2, result[0].len());

    assert_eq!(Channel::GPIO4, result[0][0].channel);
    assert_eq!(26333, result[0][0].voltage);

    assert_eq!(Channel::GPIO9, result[0][1].channel);
    assert_eq!(24970, result[0][1].voltage);
}

#[test]
fn test_read_voltages_gpio_group_5() {
    let bus = BusMockBuilder::new()
        // Register B
        .expect_command(0b0000_0000, 0b0000_1110, 0x72, 0x9A)
        .expect_register_read(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_voltages(GPIOSelection::Group5).unwrap();
    assert_eq!(1, result[0].len());

    assert_eq!(Channel::GPIO5, result[0][0].channel);
    assert_eq!(7538, result[0][0].voltage);
}

#[test]
fn test_read_voltages_gpio_group_6() {
    let bus = BusMockBuilder::new()
        // Register B
        .expect_command(0b0000_0000, 0b0000_1110, 0x72, 0x9A)
        .expect_register_read(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_voltages(GPIOSelection::Group6).unwrap();
    assert_eq!(1, result[0].len());

    assert_eq!(Channel::SecondReference, result[0][0].channel);
    assert_eq!(7330, result[0][0].voltage);
}

#[test]
fn test_read_voltages_gpio_all() {
    let bus = BusMockBuilder::new()
        // Register A
        .expect_command(0b0000_0000, 0b0000_1100, 0xEF, 0xCC)
        .expect_register_read(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C])
        // Register C
        .expect_command(0b0000_0000, 0b0000_1101, 0x64, 0xFE)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        // Register D
        .expect_command(0b0000_0000, 0b0000_1110, 0x72, 0x9A)
        .expect_register_read(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94])
        // Register B
        .expect_command(0b0000_0000, 0b0000_1111, 0xF9, 0xA8)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(4));

    let mut result = monitor.read_voltages(GPIOSelection::All).unwrap();
    assert_eq!(10, result[0].len());
    result[0].sort_by_key(|element| element.channel);

    assert_eq!(Channel::GPIO1, result[0][0].channel);
    assert_eq!(24979, result[0][0].voltage);

    assert_eq!(Channel::GPIO6, result[0][5].channel);
    assert_eq!(25441, result[0][5].voltage);

    assert_eq!(Channel::GPIO2, result[0][1].channel);
    assert_eq!(7867, result[0][1].voltage);

    assert_eq!(Channel::GPIO7, result[0][6].channel);
    assert_eq!(7869, result[0][6].voltage);

    assert_eq!(Channel::GPIO3, result[0][2].channel);
    assert_eq!(8878, result[0][2].voltage);

    assert_eq!(Channel::GPIO8, result[0][7].channel);
    assert_eq!(8932, result[0][7].voltage);

    assert_eq!(Channel::GPIO4, result[0][3].channel);
    assert_eq!(26333, result[0][3].voltage);

    assert_eq!(Channel::GPIO9, result[0][8].channel);
    assert_eq!(24970, result[0][8].voltage);

    assert_eq!(Channel::GPIO5, result[0][4].channel);
    assert_eq!(7538, result[0][4].voltage);

    assert_eq!(Channel::SecondReference, result[0][9].channel);
    assert_eq!(7330, result[0][9].voltage);
}

#[test]
fn test_read_voltages_cell_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = MockSPIBus::new();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_voltages(CellSelection::Group1);
    match result.unwrap_err() {
        Error::CSPinError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_voltages_cell_transfer_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_voltages(CellSelection::Group1);
    match result.unwrap_err() {
        Error::TransferError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

/// Creates a pin mock for no polling method
fn get_cs_no_polling(call_count: usize) -> MockPin {
    let mut cs = MockPin::new();
    cs.expect_set_high().times(call_count).returning(move || Ok(()));
    cs.expect_set_low().times(call_count).returning(move || Ok(()));

    cs
}

#[test]
fn test_ltc6813_read_overlap_results() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1000, 0x5E, 0x52)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        .expect_command(0b0000_0000, 0b0000_1001, 0xD5, 0x60)
        .expect_register_read(&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(2));

    let result = monitor.read_overlap_result().unwrap();
    assert_eq!(25441, result[0][0]);
    assert_eq!(7869, result[0][1]);
    assert_eq!(25822, result[0][2]);
    assert_eq!(8591, result[0][3]);
}

#[test]
fn test_ltc6812_read_overlap_results() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1000, 0x5E, 0x52)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        .expect_command(0b0000_0000, 0b0000_1001, 0xD5, 0x60)
        .expect_register_read(&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6812(bus, get_cs_no_polling(2));

    let result = monitor.read_overlap_result().unwrap();
    assert_eq!(25441, result[0][0]);
    assert_eq!(7869, result[0][1]);
    assert_eq!(25822, result[0][2]);
    assert_eq!(8591, result[0][3]);
}

#[test]
fn test_ltc6811_read_overlap_results() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1000, 0x5E, 0x52)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6811(bus, get_cs_no_polling(1));

    let result = monitor.read_overlap_result().unwrap();
    assert_eq!(25441, result[0][0]);
    assert_eq!(7869, result[0][1]);
    assert_eq!(0, result[0][2]);
    assert_eq!(0, result[0][3]);
}

#[test]
fn test_ltc6810_read_overlap_results() {
    let bus = BusMockBuilder::new().into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6810(bus, get_cs_no_polling(0));

    let result = monitor.read_overlap_result().unwrap();
    assert_eq!(0, result[0][0]);
    assert_eq!(0, result[0][1]);
    assert_eq!(0, result[0][2]);
    assert_eq!(0, result[0][3]);
}

#[test]
fn test_read_overlap_results_multiple_devices() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1000, 0x5E, 0x52)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        .expect_register_read(&[0x53, 0x64, 0x76, 0x1E, 0xB9, 0x1E, 0x1B, 0xC6])
        .expect_command(0b0000_0000, 0b0000_1001, 0xD5, 0x60)
        .expect_register_read(&[0xA2, 0x62, 0x05, 0x1F, 0xC9, 0x20, 0xEE, 0x94])
        .expect_register_read(&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 2> = LTC681X::ltc6813(bus, get_cs_no_polling(2));

    let result = monitor.read_overlap_result().unwrap();

    assert_eq!(24970, result[0][0]);
    assert_eq!(25683, result[1][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(7798, result[1][1]);

    assert_eq!(25250, result[0][2]);
    assert_eq!(25822, result[1][2]);
    assert_eq!(7941, result[0][3]);
    assert_eq!(8591, result[1][3]);
}

#[test]
fn test_read_overlap_result_pec_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1000, 0x5E, 0x52)
        .expect_register_read(&[0x2A, 0x63, 0x8E, 0x1E, 0xEC, 0x1F, 0x11, 0x0D])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_overlap_result();
    match result.unwrap_err() {
        Error::ChecksumMismatch => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_overlap_result_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = MockSPIBus::new();

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_overlap_result();
    match result.unwrap_err() {
        Error::CSPinError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_overlap_result_transfer_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_overlap_result();
    match result.unwrap_err() {
        Error::TransferError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}
