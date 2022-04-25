use crate::ltc6813::{CellSelection, GPIOSelection};
use crate::mocks::{BusError, BusMockBuilder, MockPin, MockSPIBus, PinError};
use crate::monitor::{ADCMode, AuxiliaryRegister, CellVoltageRegister, Error, LTC681XClient, PollClient, LTC681X};
use crate::pec15::PEC15;

#[test]
fn test_start_conv_cells_acc_modes() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0011, 0b0110_0000, 0xf4, 0x6c)
        .expect_command(0b0000_0010, 0b1110_0000, 0x38, 0x06)
        .expect_command(0b0000_0011, 0b1110_0000, 0xb0, 0x4a)
        .expect_command(0b0000_0010, 0b0110_0000, 0x7c, 0x20)
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(4));
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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(7));
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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(2));
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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false).unwrap();
}

#[test]
fn test_start_conv_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = MockSPIBus::new();
    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(4));
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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(7));
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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();
    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All).unwrap();
}

#[test]
fn test_start_conv_gpio_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = MockSPIBus::new();
    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All);
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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();
    assert!(monitor.adc_ready().unwrap());
}

#[test]
fn test_sdo_polling_not_ready() {
    let cs = MockPin::new();

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Ok(&[0x00]));

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();
    assert!(!monitor.adc_ready().unwrap());
}

#[test]
fn test_sdo_polling_transfer_error() {
    let cs = MockPin::new();
    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();

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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs).enable_sdo_polling();

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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_cell_voltages(CellVoltageRegister::RegisterA).unwrap();
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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_cell_voltages(CellVoltageRegister::RegisterB).unwrap();
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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_cell_voltages(CellVoltageRegister::RegisterC).unwrap();
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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_cell_voltages(CellVoltageRegister::RegisterD).unwrap();
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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_cell_voltages(CellVoltageRegister::RegisterE).unwrap();
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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_cell_voltages(CellVoltageRegister::RegisterF).unwrap();
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

    let mut monitor: LTC681X<_, _, _, _, _, 3> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_cell_voltages(CellVoltageRegister::RegisterD).unwrap();

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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_cell_voltages(CellVoltageRegister::RegisterF);
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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_cell_voltages(CellVoltageRegister::RegisterF);
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

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_cell_voltages(CellVoltageRegister::RegisterF);
    match result.unwrap_err() {
        Error::TransferError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_aux_voltages_register_a() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1100, 0xEF, 0xCC)
        .expect_register_read(&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_aux_voltages(AuxiliaryRegister::RegisterA).unwrap();
    assert_eq!(24979, result[0][0]);
    assert_eq!(7867, result[0][1]);
    assert_eq!(8878, result[0][2]);
}

#[test]
fn test_read_aux_voltages_register_b() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1110, 0x72, 0x9A)
        .expect_register_read(&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_aux_voltages(AuxiliaryRegister::RegisterB).unwrap();
    assert_eq!(26333, result[0][0]);
    assert_eq!(7538, result[0][1]);
    assert_eq!(7330, result[0][2]);
}

#[test]
fn test_read_aux_voltages_register_c() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1101, 0x64, 0xFE)
        .expect_register_read(&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_aux_voltages(AuxiliaryRegister::RegisterC).unwrap();
    assert_eq!(25441, result[0][0]);
    assert_eq!(7869, result[0][1]);
    assert_eq!(8932, result[0][2]);
}

#[test]
fn test_read_aux_voltages_register_d() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1111, 0xF9, 0xA8)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_aux_voltages(AuxiliaryRegister::RegisterD).unwrap();
    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);
}

#[test]
fn test_read_aux_voltages_multiple_devices() {
    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1111, 0xF9, 0xA8)
        .expect_register_read(&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE])
        .expect_register_read(&[0x53, 0x64, 0x76, 0x1E, 0xB9, 0x1E, 0x1B, 0xC6])
        .expect_register_read(&[0xA2, 0x62, 0x05, 0x1F, 0xC9, 0x20, 0xEE, 0x94])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, _, 3> = LTC681X::ltc6813(bus, get_cs_no_polling(1));

    let result = monitor.read_aux_voltages(AuxiliaryRegister::RegisterD).unwrap();

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
fn test_read_aux_voltages_pec_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0000, 0b0000_1111, 0xF9, 0xA8)
        .expect_register_read(&[0x2A, 0x63, 0x8E, 0x1E, 0xEC, 0x1F, 0x11, 0x0D])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_aux_voltages(AuxiliaryRegister::RegisterD);
    match result.unwrap_err() {
        Error::ChecksumMismatch => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_aux_voltages_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = MockSPIBus::new();

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_aux_voltages(AuxiliaryRegister::RegisterD);
    match result.unwrap_err() {
        Error::CSPinError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_aux_voltages_transfer_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, _, _, _, 1> = LTC681X::ltc6813(bus, cs);

    let result = monitor.read_aux_voltages(AuxiliaryRegister::RegisterD);
    match result.unwrap_err() {
        Error::TransferError(_) => {}
        _ => panic!("Unexpected error type"),
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
