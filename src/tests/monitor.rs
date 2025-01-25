//! Tests for generic, device type independent, logic
use crate::config::{Cell, Configuration, GPIO};
use crate::ltc6810::LTC6810;
use crate::ltc6811::LTC6811;
use crate::ltc6812::LTC6812;
use crate::ltc6813::{CellSelection, Channel, GPIOSelection, Register, LTC6813};
use crate::mocks::{BusError, BusMockBuilder, DeviceMockBuilder, MockPin, MockSPIBus, MockSPIDevice, PinError};
use crate::monitor::{ADCMode, Error, LTC681XClient, PollClient, StatusGroup, LTC681X};
use alloc::string::ToString;

#[test]
fn test_start_conv_cells_acc_modes() {
    let bus = DeviceMockBuilder::new()
        .expect_command(0b0000_0011, 0b0110_0000, 0xf4, 0x6c)
        .expect_command(0b0000_0010, 0b1110_0000, 0x38, 0x06)
        .expect_command(0b0000_0011, 0b1110_0000, 0xb0, 0x4a)
        .expect_command(0b0000_0010, 0b0110_0000, 0x7c, 0x20)
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let timing = monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false).unwrap();
    assert_eq!(2343, timing.regular);
    assert_eq!(3041, timing.alternative);

    let timing = monitor.start_conv_cells(ADCMode::Fast, CellSelection::All, false).unwrap();
    assert_eq!(1121, timing.regular);
    assert_eq!(1296, timing.alternative);

    let timing = monitor.start_conv_cells(ADCMode::Filtered, CellSelection::All, false).unwrap();
    assert_eq!(201325, timing.regular);
    assert_eq!(4437, timing.alternative);

    let timing = monitor.start_conv_cells(ADCMode::Other, CellSelection::All, false).unwrap();
    assert_eq!(12816, timing.regular);
    assert_eq!(7230, timing.alternative);
}

#[test]
fn test_start_conv_cells_cell_groups() {
    let bus = DeviceMockBuilder::new()
        .expect_command(0b0000_0011, 0b0110_0000, 0xf4, 0x6c)
        .expect_command(0b0000_0011, 0b0110_0001, 0x7F, 0x5E)
        .expect_command(0b0000_0011, 0b0110_0010, 0x69, 0x3A)
        .expect_command(0b0000_0011, 0b0110_0011, 0xE2, 0x8)
        .expect_command(0b0000_0011, 0b0110_0100, 0x45, 0xF2)
        .expect_command(0b0000_0011, 0b0110_0101, 0xCE, 0xC0)
        .expect_command(0b0000_0011, 0b0110_0110, 0xD8, 0xA4)
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let timing = monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false).unwrap();
    assert_eq!(2343, timing.regular);
    assert_eq!(3041, timing.alternative);

    let timing = monitor.start_conv_cells(ADCMode::Normal, CellSelection::Group1, false).unwrap();
    assert_eq!(407, timing.regular);
    assert_eq!(523, timing.alternative);

    let timing = monitor.start_conv_cells(ADCMode::Normal, CellSelection::Group2, false).unwrap();
    assert_eq!(407, timing.regular);
    assert_eq!(523, timing.alternative);

    let timing = monitor.start_conv_cells(ADCMode::Normal, CellSelection::Group3, false).unwrap();
    assert_eq!(407, timing.regular);
    assert_eq!(523, timing.alternative);

    let timing = monitor.start_conv_cells(ADCMode::Normal, CellSelection::Group4, false).unwrap();
    assert_eq!(407, timing.regular);
    assert_eq!(523, timing.alternative);

    let timing = monitor.start_conv_cells(ADCMode::Normal, CellSelection::Group5, false).unwrap();
    assert_eq!(407, timing.regular);
    assert_eq!(523, timing.alternative);

    let timing = monitor.start_conv_cells(ADCMode::Normal, CellSelection::Group6, false).unwrap();
    assert_eq!(407, timing.regular);
    assert_eq!(523, timing.alternative);
}

#[test]
fn test_start_conv_permit_charging() {
    let bus = DeviceMockBuilder::new()
        .expect_command(0b0000_0011, 0b0110_0000, 0xf4, 0x6c)
        .expect_command(0b0000_0011, 0b0111_0000, 0xAF, 0x42)
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);
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

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);
    monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false).unwrap();
}

#[test]
fn test_start_conv_cells_sdo_polling_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = BusMockBuilder::new().into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);
    let result = monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false);

    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_start_conv_transfer_error() {
    let mut bus = MockSPIDevice::new();
    bus.expect_transaction().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.start_conv_cells(ADCMode::Normal, CellSelection::All, false);
    match result.unwrap_err() {
        Error::BusError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_start_conv_gpio_acc_modes() {
    let bus = DeviceMockBuilder::new()
        .expect_command(0b0000_0101, 0b0110_0000, 0xD3, 0xA0)
        .expect_command(0b0000_0100, 0b1110_0000, 0x1F, 0xCA)
        .expect_command(0b0000_0101, 0b1110_0000, 0x97, 0x86)
        .expect_command(0b0000_0100, 0b0110_0000, 0x5B, 0xEC)
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);
    let timing = monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All).unwrap();
    assert_eq!(3862, timing.regular);
    assert_eq!(5025, timing.alternative);

    let timing = monitor.start_conv_gpio(ADCMode::Fast, GPIOSelection::All).unwrap();
    assert_eq!(1825, timing.regular);
    assert_eq!(2116, timing.alternative);

    let timing = monitor.start_conv_gpio(ADCMode::Filtered, GPIOSelection::All).unwrap();
    assert_eq!(335_498, timing.regular);
    assert_eq!(7_353, timing.alternative);

    let timing = monitor.start_conv_gpio(ADCMode::Other, GPIOSelection::All).unwrap();
    assert_eq!(21_316, timing.regular);
    assert_eq!(12_007, timing.alternative);
}

#[test]
fn test_start_conv_gpio_groups() {
    let bus = DeviceMockBuilder::new()
        .expect_command(0b0000_0101, 0b0110_0000, 0xD3, 0xA0)
        .expect_command(0b0000_0101, 0b0110_0001, 0x58, 0x92)
        .expect_command(0b0000_0101, 0b0110_0010, 0x4E, 0xF6)
        .expect_command(0b0000_0101, 0b0110_0011, 0xC5, 0xC4)
        .expect_command(0b0000_0101, 0b0110_0100, 0x62, 0x3E)
        .expect_command(0b0000_0101, 0b0110_0101, 0xE9, 0xC)
        .expect_command(0b0000_0101, 0b0110_0110, 0xFF, 0x68)
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);
    let timing = monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All).unwrap();
    assert_eq!(3862, timing.regular);
    assert_eq!(5025, timing.alternative);

    let timing = monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group1).unwrap();
    assert_eq!(788, timing.regular);
    assert_eq!(1000, timing.alternative);

    let timing = monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group2).unwrap();
    assert_eq!(788, timing.regular);
    assert_eq!(1000, timing.alternative);

    let timing = monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group3).unwrap();
    assert_eq!(788, timing.regular);
    assert_eq!(1000, timing.alternative);

    let timing = monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group4).unwrap();
    assert_eq!(788, timing.regular);
    assert_eq!(1000, timing.alternative);

    let timing = monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group5).unwrap();
    assert_eq!(403, timing.regular);
    assert_eq!(520, timing.alternative);

    let timing = monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::Group6).unwrap();
    assert_eq!(403, timing.regular);
    assert_eq!(520, timing.alternative);
}

#[test]
fn test_start_conv_gpio_sdo_polling() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0101, 0b0110_0000, 0xD3, 0xA0)
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);
    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All).unwrap();
}

#[test]
fn test_start_conv_gpio_sdo_polling_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = BusMockBuilder::new().into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);
    let result = monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All);

    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_start_conv_gpio_transfer_error() {
    let mut bus = MockSPIDevice::new();
    bus.expect_transaction().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All);
    match result.unwrap_err() {
        Error::BusError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_start_overlap_measurement_acc_modes() {
    let bus = DeviceMockBuilder::new()
        .expect_command(0b0000_0011, 0b0000_0001, 0x2E, 0x88)
        .expect_command(0b0000_0010, 0b1000_0001, 0xE2, 0xE2)
        .expect_command(0b0000_0011, 0b1000_0001, 0x6A, 0xAE)
        .expect_command(0b0000_0010, 0b0000_0001, 0xA6, 0xC4)
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);
    monitor.start_overlap_measurement(ADCMode::Normal, false).unwrap();
    monitor.start_overlap_measurement(ADCMode::Fast, false).unwrap();
    monitor.start_overlap_measurement(ADCMode::Filtered, false).unwrap();
    monitor.start_overlap_measurement(ADCMode::Other, false).unwrap();
}

#[test]
fn test_start_overlap_measurement_dcp() {
    let bus = DeviceMockBuilder::new()
        .expect_command(0b0000_0011, 0b0001_0001, 0x75, 0xA6)
        .expect_command(0b0000_0011, 0b0000_0001, 0x2E, 0x88)
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);
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

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);
    monitor.start_overlap_measurement(ADCMode::Normal, false).unwrap();
}

#[test]
fn test_start_overlap_measurement_sdo_polling_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = BusMockBuilder::new().into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);
    let result = monitor.start_overlap_measurement(ADCMode::Normal, false);

    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_start_overlap_measurement_transfer_error() {
    let mut bus = MockSPIDevice::new();
    bus.expect_transaction().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.start_overlap_measurement(ADCMode::Normal, false);
    match result.unwrap_err() {
        Error::BusError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_measure_internal_parameters_acc_modes() {
    let bus = DeviceMockBuilder::new()
        .expect_command(0b0000_0101, 0b0110_1000, 0x3B, 0xAE)
        .expect_command(0b0000_0100, 0b1110_1000, 0xF7, 0xC4)
        .expect_command(0b0000_0101, 0b1110_1000, 0x7F, 0x88)
        .expect_command(0b0000_0100, 0b0110_1000, 0xB3, 0xE2)
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);
    let timing = monitor.measure_internal_parameters(ADCMode::Normal, StatusGroup::All).unwrap();
    assert_eq!(1_600, timing.regular);
    assert_eq!(2_000, timing.alternative);

    let timing = monitor.measure_internal_parameters(ADCMode::Fast, StatusGroup::All).unwrap();
    assert_eq!(742, timing.regular);
    assert_eq!(858, timing.alternative);

    let timing = monitor
        .measure_internal_parameters(ADCMode::Filtered, StatusGroup::All)
        .unwrap();
    assert_eq!(134_000, timing.regular);
    assert_eq!(3_000, timing.alternative);

    let timing = monitor.measure_internal_parameters(ADCMode::Other, StatusGroup::All).unwrap();
    assert_eq!(8_500, timing.regular);
    assert_eq!(4_800, timing.alternative);
}

#[test]
fn test_measure_internal_parameters_status_groups() {
    let bus = DeviceMockBuilder::new()
        .expect_command(0b0000_0101, 0b0110_1000, 0x3B, 0xAE)
        .expect_command(0b0000_0101, 0b0110_1001, 0xB0, 0x9C)
        .expect_command(0b0000_0101, 0b0110_1010, 0xA6, 0xF8)
        .expect_command(0b0000_0101, 0b0110_1011, 0x2D, 0xCA)
        .expect_command(0b0000_0101, 0b0110_1100, 0x8A, 0x30)
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);
    let timing = monitor.measure_internal_parameters(ADCMode::Normal, StatusGroup::All).unwrap();
    assert_eq!(1_600, timing.regular);
    assert_eq!(2_000, timing.alternative);

    let timing = monitor
        .measure_internal_parameters(ADCMode::Normal, StatusGroup::CellSum)
        .unwrap();
    assert_eq!(403, timing.regular);
    assert_eq!(520, timing.alternative);

    let timing = monitor
        .measure_internal_parameters(ADCMode::Normal, StatusGroup::Temperature)
        .unwrap();
    assert_eq!(403, timing.regular);
    assert_eq!(520, timing.alternative);

    let timing = monitor
        .measure_internal_parameters(ADCMode::Normal, StatusGroup::AnalogVoltage)
        .unwrap();
    assert_eq!(403, timing.regular);
    assert_eq!(520, timing.alternative);

    let timing = monitor
        .measure_internal_parameters(ADCMode::Normal, StatusGroup::DigitalVoltage)
        .unwrap();
    assert_eq!(403, timing.regular);
    assert_eq!(520, timing.alternative);
}

#[test]
fn test_measure_internal_parameters_sdo_polling() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let bus = BusMockBuilder::new()
        .expect_command(0b0000_0101, 0b0110_1000, 59, 174)
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);
    monitor.measure_internal_parameters(ADCMode::Normal, StatusGroup::All).unwrap();
}

#[test]
fn test_measure_internal_parameters_sdo_polling_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Err(PinError::Error1));

    let bus = BusMockBuilder::new().into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);
    let result = monitor.measure_internal_parameters(ADCMode::Normal, StatusGroup::All);

    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_measure_internal_parameters_transfer_error() {
    let mut bus = MockSPIDevice::new();
    bus.expect_transaction().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.measure_internal_parameters(ADCMode::Normal, StatusGroup::All);
    match result.unwrap_err() {
        Error::BusError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_sdo_polling_ready() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));
    cs.expect_set_high().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_write().times(1).returning(move |_| Ok(()));
    bus.expect_read().times(1).returning(move |buffer| {
        assert_eq!(1, buffer.len());
        buffer[0] = 0xff;

        Ok(())
    });

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);
    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All).unwrap();
    assert!(monitor.adc_ready().unwrap());
}

#[test]
fn test_sdo_polling_not_ready() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_write().times(1).returning(move |_| Ok(()));
    bus.expect_read().times(1).returning(move |buffer| {
        assert_eq!(1, buffer.len());
        buffer[0] = 0x0;

        Ok(())
    });

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);
    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All).unwrap();
    assert!(!monitor.adc_ready().unwrap());
}

#[test]
fn test_sdo_polling_transfer_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));
    cs.expect_set_high().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_write().times(1).returning(move |_| Ok(()));
    bus.expect_read().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    monitor.start_conv_gpio(ADCMode::Normal, GPIOSelection::All).unwrap();
    assert!(monitor.adc_ready().is_err());
}

#[test]
fn test_sdo_polling_cs_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));
    cs.expect_set_high().times(1).returning(move || Err(PinError::Error1));

    let mut bus = MockSPIBus::new();
    bus.expect_read().times(1).returning(move |buffer| {
        assert_eq!(1, buffer.len());
        buffer[0] = 0xff;

        Ok(())
    });

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);
    assert!(monitor.adc_ready().is_err());
}

#[test]
fn test_read_cell_voltages_register_a() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_0100,
            0x07,
            0xC2,
            [&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::CellVoltageA).unwrap();
    assert_eq!(24979, result[0][0]);
    assert_eq!(7867, result[0][1]);
    assert_eq!(8878, result[0][2]);
}

#[test]
fn test_read_cell_voltages_register_b() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_0110,
            0x9A,
            0x94,
            [&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::CellVoltageB).unwrap();
    assert_eq!(26333, result[0][0]);
    assert_eq!(7538, result[0][1]);
    assert_eq!(7330, result[0][2]);
}

#[test]
fn test_read_cell_voltages_register_c() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            [&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::CellVoltageC).unwrap();
    assert_eq!(25441, result[0][0]);
    assert_eq!(7869, result[0][1]);
    assert_eq!(8932, result[0][2]);
}

#[test]
fn test_read_cell_voltages_register_d() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1010,
            0xC3,
            0x4,
            [&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::CellVoltageD).unwrap();
    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);
}

#[test]
fn test_read_cell_voltages_register_e() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1001,
            0xD5,
            0x60,
            [&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::CellVoltageE).unwrap();
    assert_eq!(25822, result[0][0]);
    assert_eq!(8591, result[0][1]);
    assert_eq!(8586, result[0][2]);
}

#[test]
fn test_read_cell_voltages_register_f() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1011,
            0x48,
            0x36,
            [&[0x00, 0x63, 0x2F, 0x1F, 0x8B, 0x1F, 0xC1, 0x68]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::CellVoltageF).unwrap();
    assert_eq!(25344, result[0][0]);
    assert_eq!(7983, result[0][1]);
    assert_eq!(8075, result[0][2]);
}

#[test]
fn test_read_cell_voltages_multiple_devices() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1010,
            0xC3,
            0x4,
            [
                &[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE],
                &[0x53, 0x64, 0x76, 0x1E, 0xB9, 0x1E, 0x1B, 0xC6],
                &[0xA2, 0x62, 0x05, 0x1F, 0xC9, 0x20, 0xEE, 0x94],
            ],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, _, 3> = LTC681X::ltc6813(bus);

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
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1011,
            0x48,
            0x36,
            [&[0x2A, 0x63, 0x8E, 0x1E, 0xEC, 0x1F, 0x11, 0x0D]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::CellVoltageF);
    match result.unwrap_err() {
        Error::ChecksumMismatch => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_cell_voltages_transfer_error() {
    let mut bus = MockSPIDevice::new();
    bus.expect_transaction().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::CellVoltageF);
    match result.unwrap_err() {
        Error::BusError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_register_aux_a() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1100,
            0xEF,
            0xCC,
            [&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::AuxiliaryA).unwrap();
    assert_eq!(24979, result[0][0]);
    assert_eq!(7867, result[0][1]);
    assert_eq!(8878, result[0][2]);
}

#[test]
fn test_read_register_aux_b() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1110,
            0x72,
            0x9A,
            [&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::AuxiliaryB).unwrap();
    assert_eq!(26333, result[0][0]);
    assert_eq!(7538, result[0][1]);
    assert_eq!(7330, result[0][2]);
}

#[test]
fn test_read_register_aux_c() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1101,
            0x64,
            0xFE,
            [&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::AuxiliaryC).unwrap();
    assert_eq!(25441, result[0][0]);
    assert_eq!(7869, result[0][1]);
    assert_eq!(8932, result[0][2]);
}

#[test]
fn test_read_register_aux_d() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1111,
            0xF9,
            0xA8,
            [&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::AuxiliaryD).unwrap();
    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);
}

#[test]
fn test_read_register_status_a() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0001_0000,
            0xED,
            0x72,
            [&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::StatusA).unwrap();
    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);
}

#[test]
fn test_read_register_status_b() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0001_0010,
            0x70,
            0x24,
            [&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::StatusB).unwrap();
    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);
}

#[test]
fn test_read_register_conf_a() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_0010,
            0x2B,
            0xA,
            [&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::ConfigurationA).unwrap();
    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);
}

#[test]
fn test_read_register_conf_b() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0010_0110,
            0x2C,
            0xC8,
            [&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::ConfigurationB).unwrap();
    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);
}

#[test]
fn test_read_register_multiple_devices() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1111,
            0xF9,
            0xA8,
            [
                &[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE],
                &[0x53, 0x64, 0x76, 0x1E, 0xB9, 0x1E, 0x1B, 0xC6],
                &[0xA2, 0x62, 0x05, 0x1F, 0xC9, 0x20, 0xEE, 0x94],
            ],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, _, 3> = LTC681X::ltc6813(bus);

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
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1111,
            0xF9,
            0xA8,
            [&[0x2A, 0x63, 0x8E, 0x1E, 0xEC, 0x1F, 0x11, 0x0D]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::AuxiliaryD);
    match result.unwrap_err() {
        Error::ChecksumMismatch => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_register_transfer_error() {
    let mut bus = MockSPIDevice::new();
    bus.expect_transaction().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_register(Register::AuxiliaryD);
    match result.unwrap_err() {
        Error::BusError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_register_sdo_polling() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Ok(()));
    cs.expect_set_high().times(1).returning(|| Ok(()));

    let bus = BusMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0010_0110,
            0x2C,
            0xC8,
            &[[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.read_register(Register::ConfigurationB).unwrap();
    assert_eq!(24970, result[0][0]);
    assert_eq!(8033, result[0][1]);
    assert_eq!(8655, result[0][2]);
}

#[test]
fn test_read_register_sdo_polling_cs_low_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Err(PinError::Error1));

    let bus = BusMockBuilder::new().into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.read_register(Register::ConfigurationB);
    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_register_sdo_polling_cs_high_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Ok(()));
    cs.expect_set_high().times(1).returning(|| Err(PinError::Error1));

    let bus = BusMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0010_0110,
            0x2C,
            0xC8,
            &[[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.read_register(Register::ConfigurationB);
    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_write_register_conf_a() {
    let bus = DeviceMockBuilder::new()
        .expect_register_write(&[&[
            0b0000_0000,
            0b0000_0001,
            0x3D,
            0x6E,
            0b1111_1000,
            0b0000_0100,
            0b0000_1000,
            0b0001_0000,
            0b0010_0000,
            0b0100_0000,
            0xB,
            0x24,
        ]])
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let mut data = [0x0; 6];
    data[0] = 0b1111_1000;
    data[1] = 0b0000_0100;
    data[2] = 0b0000_1000;
    data[3] = 0b0001_0000;
    data[4] = 0b0010_0000;
    data[5] = 0b0100_0000;

    monitor.write_register(Register::ConfigurationA, [data]).unwrap();
}

#[test]
fn test_write_register_conf_b() {
    let bus = DeviceMockBuilder::new()
        .expect_register_write(&[&[
            0b0000_0000,
            0b0010_0100,
            0xB1,
            0x9E,
            0b0000_0001,
            0b0000_0110,
            0b0000_1000,
            0b0011_0000,
            0b0010_1000,
            0b0100_1000,
            0x43,
            0x50,
        ]])
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let mut data = [0x0; 6];
    data[0] = 0b0000_0001;
    data[1] = 0b0000_0110;
    data[2] = 0b0000_1000;
    data[3] = 0b0011_0000;
    data[4] = 0b0010_1000;
    data[5] = 0b0100_1000;

    monitor.write_register(Register::ConfigurationB, [data]).unwrap();
}

#[test]
fn test_write_register_multiple_devices() {
    let bus = DeviceMockBuilder::new()
        .expect_register_write(&[
            &[
                0b0000_0000,
                0b0010_0100,
                0xB1,
                0x9E,
                0x1,
                0x2,
                0x3,
                0x4,
                0x5,
                0x6,
                0x22,
                0xEE,
            ],
            &[0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0x28, 0xC0],
        ])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, 2> = LTC681X::ltc6813(bus);

    let data1 = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6];
    let data2 = [0x7, 0x8, 0x9, 0xA, 0xB, 0xC];

    monitor.write_register(Register::ConfigurationB, [data1, data2]).unwrap();
}

#[test]
fn test_write_register_transfer_error() {
    let mut bus = MockSPIDevice::new();
    bus.expect_transaction().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.write_register(Register::ConfigurationA, [[0x0; 6]]);
    match result.unwrap_err() {
        Error::BusError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_write_register_sdo_polling() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Ok(()));
    cs.expect_set_high().times(1).returning(|| Ok(()));

    let bus = BusMockBuilder::new()
        .expect_register_write(&[
            0b0000_0000,
            0b0000_0001,
            0x3D,
            0x6E,
            0b1111_1000,
            0b0000_0100,
            0b0000_1000,
            0b0001_0000,
            0b0010_0000,
            0b0100_0000,
            0xB,
            0x24,
        ])
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let mut data = [0x0; 6];
    data[0] = 0b1111_1000;
    data[1] = 0b0000_0100;
    data[2] = 0b0000_1000;
    data[3] = 0b0001_0000;
    data[4] = 0b0010_0000;
    data[5] = 0b0100_0000;

    monitor.write_register(Register::ConfigurationA, [data]).unwrap();
}

#[test]
fn test_write_register_sdo_polling_cs_low_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Err(PinError::Error1));

    let bus = BusMockBuilder::new().into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.write_register(Register::ConfigurationA, [[0x0; 6]]);
    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_write_register_sdo_polling_cs_high_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Ok(()));
    cs.expect_set_high().times(1).returning(|| Err(PinError::Error1));

    let mut bus = MockSPIBus::new();
    bus.expect_write().times(1).returning(|_| Ok(()));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.write_register(Register::ConfigurationA, [[0x0; 6]]);
    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_write_configuration_correct_data() {
    let bus = DeviceMockBuilder::new()
        .expect_register_write(&[&[
            0b0000_0000,
            0b0000_0001,
            0x3D,
            0x6E,
            0b1111_1000,
            0b0101_0010,
            0b1111_0111,
            0b1010_0111,
            0b0000_0000,
            0b0000_0000,
            0x10,
            0x6C,
        ]])
        .expect_register_write(&[&[
            0b0000_0000,
            0b0010_0100,
            0xB1,
            0x9E,
            0b0001_1111,
            0b0000_0001,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0x2,
            0x5C,
        ]])
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let mut config = Configuration::default();
    config.set_ov_comp_voltage(4_300_000).unwrap();
    config.set_uv_comp_voltage(3_000_000).unwrap();
    config.discharge_cell(Cell::Cell13);
    config.discharge_cell(Cell::Cell17);

    monitor.write_configuration([config]).unwrap();
}

#[test]
fn test_write_configuration_multiple_devices() {
    let bus = DeviceMockBuilder::new()
        .expect_register_write(&[
            &[
                0b0000_0000,
                0b0000_0001,
                0x3D,
                0x6E,
                0b1111_1000,
                0b1110_0001,
                0b0100_0100,
                0b1001_1100,
                0b0000_0000,
                0b0000_1000,
                0x66,
                0xE0,
            ],
            &[
                0b1010_1000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0011_1000,
                0b0000_0000,
                0x72,
                0x5E,
            ],
        ])
        .expect_register_write(&[
            &[
                0b0000_0000,
                0b0010_0100,
                0xB1,
                0x9E,
                0b0000_1111,
                0b0000_0010,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0xA,
                0xFA,
            ],
            &[
                0b0110_1101,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0x13,
                0xD6,
            ],
        ])
        .into_mock();

    let mut monitor: LTC681X<_, _, _, 2> = LTC681X::ltc6813(bus);

    let mut config1 = Configuration::default();
    config1.set_ov_comp_voltage(4_000_000).unwrap();
    config1.set_uv_comp_voltage(2_000_000).unwrap();
    config1.discharge_cell(Cell::Cell12);
    config1.discharge_cell(Cell::Cell18);

    let mut config2 = Configuration::default();
    config2.enable_gpio_pull_down(GPIO::GPIO2);
    config2.enable_gpio_pull_down(GPIO::GPIO4);
    config2.enable_gpio_pull_down(GPIO::GPIO7);
    config2.discharge_cell(Cell::Cell4);
    config2.discharge_cell(Cell::Cell5);
    config2.discharge_cell(Cell::Cell6);
    config2.discharge_cell(Cell::Cell14);
    config2.discharge_cell(Cell::Cell15);

    monitor.write_configuration([config1, config2]).unwrap();
}

#[test]
fn test_write_configuration_ltc6810() {
    let bus = DeviceMockBuilder::new()
        .expect_register_write(&[&[
            0b0000_0000,
            0b0000_0001,
            0x3D,
            0x6E,
            0b1111_1000,
            0b0101_0010,
            0b1111_0111,
            0b1010_0111,
            0b0000_0000,
            0b0000_0000,
            0x10,
            0x6C,
        ]])
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6810, 1> = LTC681X::ltc6810(bus);

    let mut config = Configuration::default();
    config.set_ov_comp_voltage(4_300_000).unwrap();
    config.set_uv_comp_voltage(3_000_000).unwrap();
    config.discharge_cell(Cell::Cell13);
    config.discharge_cell(Cell::Cell17);

    monitor.write_configuration([config]).unwrap();
}

#[test]
fn test_write_configuration_transfer_error() {
    let mut bus = MockSPIDevice::new();
    bus.expect_transaction().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.write_configuration([Configuration::default()]);
    match result.unwrap_err() {
        Error::BusError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_write_configuration_sdo_polling() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(2).returning(|| Ok(()));
    cs.expect_set_high().times(2).returning(|| Ok(()));

    let bus = BusMockBuilder::new()
        .expect_register_write(&[
            0b0000_0000,
            0b0000_0001,
            0x3D,
            0x6E,
            0b1111_1000,
            0b0101_0010,
            0b1111_0111,
            0b1010_0111,
            0b0000_0000,
            0b0000_0000,
            0x10,
            0x6C,
        ])
        .expect_register_write(&[
            0b0000_0000,
            0b0010_0100,
            0xB1,
            0x9E,
            0b0001_1111,
            0b0000_0001,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0x2,
            0x5C,
        ])
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let mut config = Configuration::default();
    config.set_ov_comp_voltage(4_300_000).unwrap();
    config.set_uv_comp_voltage(3_000_000).unwrap();
    config.discharge_cell(Cell::Cell13);
    config.discharge_cell(Cell::Cell17);

    monitor.write_configuration([config]).unwrap();
}

#[test]
fn test_write_configuration_sdo_polling_cs_low_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Err(PinError::Error1));

    let bus = BusMockBuilder::new().into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.write_configuration([Configuration::default()]);
    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_write_configuration_sdo_polling_cs_high_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Ok(()));
    cs.expect_set_high().times(1).returning(|| Err(PinError::Error1));

    let mut bus = MockSPIBus::new();
    bus.expect_write().times(1).returning(move |_| Ok(()));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.write_configuration([Configuration::default()]);
    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_voltages_cell_group_1() {
    let bus = DeviceMockBuilder::new()
        // Register A
        .expect_register_read(
            0b0000_0000,
            0b0000_0100,
            0x07,
            0xC2,
            [&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]],
        )
        // Register C
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            [&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        // Register E
        .expect_register_read(
            0b0000_0000,
            0b0000_1001,
            0xD5,
            0x60,
            [&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

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
    let bus = DeviceMockBuilder::new()
        // Register A
        .expect_register_read(
            0b0000_0000,
            0b0000_0100,
            0x07,
            0xC2,
            [&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]],
        )
        // Register C
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            [&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        // Register E
        .expect_register_read(
            0b0000_0000,
            0b0000_1001,
            0xD5,
            0x60,
            [&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

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
    let bus = DeviceMockBuilder::new()
        // Register A
        .expect_register_read(
            0b0000_0000,
            0b0000_0100,
            0x07,
            0xC2,
            [&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]],
        )
        // Register C
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            [&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        // Register E
        .expect_register_read(
            0b0000_0000,
            0b0000_1001,
            0xD5,
            0x60,
            [&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

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
    let bus = DeviceMockBuilder::new()
        // Register B
        .expect_register_read(
            0b0000_0000,
            0b0000_0110,
            0x9A,
            0x94,
            [&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94]],
        )
        // Register D
        .expect_register_read(
            0b0000_0000,
            0b0000_1010,
            0xC3,
            0x4,
            [&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        // Register F
        .expect_register_read(
            0b0000_0000,
            0b0000_1011,
            0x48,
            0x36,
            [&[0x00, 0x63, 0x2F, 0x1F, 0x8B, 0x1F, 0xC1, 0x68]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

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
    let bus = DeviceMockBuilder::new()
        // Register B
        .expect_register_read(
            0b0000_0000,
            0b0000_0110,
            0x9A,
            0x94,
            [&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94]],
        )
        // Register D
        .expect_register_read(
            0b0000_0000,
            0b0000_1010,
            0xC3,
            0x4,
            [&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        // Register F
        .expect_register_read(
            0b0000_0000,
            0b0000_1011,
            0x48,
            0x36,
            [&[0x00, 0x63, 0x2F, 0x1F, 0x8B, 0x1F, 0xC1, 0x68]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

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
    let bus = DeviceMockBuilder::new()
        // Register B
        .expect_register_read(
            0b0000_0000,
            0b0000_0110,
            0x9A,
            0x94,
            [&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94]],
        )
        // Register D
        .expect_register_read(
            0b0000_0000,
            0b0000_1010,
            0xC3,
            0x4,
            [&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        // Register F
        .expect_register_read(
            0b0000_0000,
            0b0000_1011,
            0x48,
            0x36,
            [&[0x00, 0x63, 0x2F, 0x1F, 0x8B, 0x1F, 0xC1, 0x68]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

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
    let bus = DeviceMockBuilder::new()
        // Register A
        .expect_register_read(
            0b0000_0000,
            0b0000_0100,
            0x07,
            0xC2,
            [&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]],
        )
        // Register C
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            [&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        // Register E
        .expect_register_read(
            0b0000_0000,
            0b0000_1001,
            0xD5,
            0x60,
            [&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA]],
        )
        // Register B
        .expect_register_read(
            0b0000_0000,
            0b0000_0110,
            0x9A,
            0x94,
            [&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94]],
        )
        // Register D
        .expect_register_read(
            0b0000_0000,
            0b0000_1010,
            0xC3,
            0x4,
            [&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        // Register F
        .expect_register_read(
            0b0000_0000,
            0b0000_1011,
            0x48,
            0x36,
            [&[0x00, 0x63, 0x2F, 0x1F, 0x8B, 0x1F, 0xC1, 0x68]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

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
    let bus = DeviceMockBuilder::new()
        // Register A
        .expect_register_read(
            0b0000_0000,
            0b0000_0100,
            0x07,
            0xC2,
            [
                &[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C],
                &[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94],
            ],
        )
        // Register C
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            [
                &[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42],
                &[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE],
            ],
        )
        // Register E
        .expect_register_read(
            0b0000_0000,
            0b0000_1001,
            0xD5,
            0x60,
            [
                &[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA],
                &[0x00, 0x63, 0x2F, 0x1F, 0x8B, 0x1F, 0xC1, 0x68],
            ],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, _, 2> = LTC681X::ltc6813(bus);

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
    let bus = DeviceMockBuilder::new()
        // Register A
        .expect_register_read(
            0b0000_0000,
            0b0000_1100,
            0xEF,
            0xCC,
            [&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]],
        )
        // Register C
        .expect_register_read(
            0b0000_0000,
            0b0000_1101,
            0x64,
            0xFE,
            [&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_voltages(GPIOSelection::Group1).unwrap();
    assert_eq!(2, result[0].len());

    assert_eq!(Channel::GPIO1, result[0][0].channel);
    assert_eq!(24979, result[0][0].voltage);

    assert_eq!(Channel::GPIO6, result[0][1].channel);
    assert_eq!(25441, result[0][1].voltage);
}

#[test]
fn test_read_voltages_gpio_group_2() {
    let bus = DeviceMockBuilder::new()
        // Register A
        .expect_register_read(
            0b0000_0000,
            0b0000_1100,
            0xEF,
            0xCC,
            [&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]],
        )
        // Register C
        .expect_register_read(
            0b0000_0000,
            0b0000_1101,
            0x64,
            0xFE,
            [&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_voltages(GPIOSelection::Group2).unwrap();
    assert_eq!(2, result[0].len());

    assert_eq!(Channel::GPIO2, result[0][0].channel);
    assert_eq!(7867, result[0][0].voltage);

    assert_eq!(Channel::GPIO7, result[0][1].channel);
    assert_eq!(7869, result[0][1].voltage);
}

#[test]
fn test_read_voltages_gpio_group_3() {
    let bus = DeviceMockBuilder::new()
        // Register A
        .expect_register_read(
            0b0000_0000,
            0b0000_1100,
            0xEF,
            0xCC,
            [&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]],
        )
        // Register C
        .expect_register_read(
            0b0000_0000,
            0b0000_1101,
            0x64,
            0xFE,
            [&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_voltages(GPIOSelection::Group3).unwrap();
    assert_eq!(2, result[0].len());

    assert_eq!(Channel::GPIO3, result[0][0].channel);
    assert_eq!(8878, result[0][0].voltage);

    assert_eq!(Channel::GPIO8, result[0][1].channel);
    assert_eq!(8932, result[0][1].voltage);
}

#[test]
fn test_read_voltages_gpio_group_4() {
    let bus = DeviceMockBuilder::new()
        // Register D
        .expect_register_read(
            0b0000_0000,
            0b0000_1110,
            0x72,
            0x9A,
            [&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94]],
        )
        // Register B
        .expect_register_read(
            0b0000_0000,
            0b0000_1111,
            0xF9,
            0xA8,
            [&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_voltages(GPIOSelection::Group4).unwrap();
    assert_eq!(2, result[0].len());

    assert_eq!(Channel::GPIO4, result[0][0].channel);
    assert_eq!(26333, result[0][0].voltage);

    assert_eq!(Channel::GPIO9, result[0][1].channel);
    assert_eq!(24970, result[0][1].voltage);
}

#[test]
fn test_read_voltages_gpio_group_5() {
    let bus = DeviceMockBuilder::new()
        // Register B
        .expect_register_read(
            0b0000_0000,
            0b0000_1110,
            0x72,
            0x9A,
            [&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_voltages(GPIOSelection::Group5).unwrap();
    assert_eq!(1, result[0].len());

    assert_eq!(Channel::GPIO5, result[0][0].channel);
    assert_eq!(7538, result[0][0].voltage);
}

#[test]
fn test_read_voltages_gpio_group_6() {
    let bus = DeviceMockBuilder::new()
        // Register B
        .expect_register_read(
            0b0000_0000,
            0b0000_1110,
            0x72,
            0x9A,
            [&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_voltages(GPIOSelection::Group6).unwrap();
    assert_eq!(1, result[0].len());

    assert_eq!(Channel::SecondReference, result[0][0].channel);
    assert_eq!(7330, result[0][0].voltage);
}

#[test]
fn test_read_voltages_gpio_all() {
    let bus = DeviceMockBuilder::new()
        // Register A
        .expect_register_read(
            0b0000_0000,
            0b0000_1100,
            0xEF,
            0xCC,
            [&[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]],
        )
        // Register C
        .expect_register_read(
            0b0000_0000,
            0b0000_1101,
            0x64,
            0xFE,
            [&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        // Register D
        .expect_register_read(
            0b0000_0000,
            0b0000_1110,
            0x72,
            0x9A,
            [&[0xDD, 0x66, 0x72, 0x1D, 0xA2, 0x1C, 0x11, 0x94]],
        )
        // Register B
        .expect_register_read(
            0b0000_0000,
            0b0000_1111,
            0xF9,
            0xA8,
            [&[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

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

    // Assert cloning is working
    let cloned_voltage = result[0][0];
    assert_eq!(24979, cloned_voltage.voltage);
}

#[test]
fn test_read_voltages_cell_transfer_error() {
    let mut bus = MockSPIDevice::new();
    bus.expect_transaction().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_voltages(CellSelection::Group1);
    match result.unwrap_err() {
        Error::BusError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_voltages_sdo_polling() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(3).returning(|| Ok(()));
    cs.expect_set_high().times(3).returning(|| Ok(()));

    let bus = BusMockBuilder::new()
        // Register A
        .expect_register_read(
            0b0000_0000,
            0b0000_0100,
            0x07,
            0xC2,
            &[[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]],
        )
        // Register C
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            &[[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        // Register E
        .expect_register_read(
            0b0000_0000,
            0b0000_1001,
            0xD5,
            0x60,
            &[[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

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
fn test_read_voltages_sdo_polling_cs_low_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Err(PinError::Error1));

    let bus = MockSPIBus::new();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.read_voltages(CellSelection::Group1);
    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_voltages_sdo_polling_cs_high_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Ok(()));
    cs.expect_set_high().times(1).returning(|| Err(PinError::Error1));

    let bus = BusMockBuilder::new()
        // Register A
        .expect_register_read(
            0b0000_0000,
            0b0000_0100,
            0x07,
            0xC2,
            &[[0x93, 0x61, 0xBB, 0x1E, 0xAE, 0x22, 0x9A, 0x1C]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.read_voltages(CellSelection::Group1);
    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_ltc6813_read_overlap_results() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            [&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        .expect_register_read(
            0b0000_0000,
            0b0000_1001,
            0xD5,
            0x60,
            [&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_overlap_result().unwrap();
    assert_eq!(25441, result[0][0]);
    assert_eq!(7869, result[0][1]);
    assert_eq!(25822, result[0][2]);
    assert_eq!(8591, result[0][3]);
}

#[test]
fn test_ltc6812_read_overlap_results() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            [&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        .expect_register_read(
            0b0000_0000,
            0b0000_1001,
            0xD5,
            0x60,
            [&[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6812, 1> = LTC681X::ltc6812(bus);

    let result = monitor.read_overlap_result().unwrap();
    assert_eq!(25441, result[0][0]);
    assert_eq!(7869, result[0][1]);
    assert_eq!(25822, result[0][2]);
    assert_eq!(8591, result[0][3]);
}

#[test]
fn test_ltc6811_read_overlap_results() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            [&[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6811, 1> = LTC681X::ltc6811(bus);

    let result = monitor.read_overlap_result().unwrap();
    assert_eq!(25441, result[0][0]);
    assert_eq!(7869, result[0][1]);
    assert_eq!(0, result[0][2]);
    assert_eq!(0, result[0][3]);
}

#[test]
fn test_ltc6810_read_overlap_results() {
    let bus = DeviceMockBuilder::new().into_mock();

    let mut monitor: LTC681X<_, _, LTC6810, 1> = LTC681X::ltc6810(bus);

    let result = monitor.read_overlap_result().unwrap();
    assert_eq!(0, result[0][0]);
    assert_eq!(0, result[0][1]);
    assert_eq!(0, result[0][2]);
    assert_eq!(0, result[0][3]);
}

#[test]
fn test_read_overlap_results_multiple_devices() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            [
                &[0x8A, 0x61, 0x61, 0x1F, 0xCF, 0x21, 0x01, 0xEE],
                &[0x53, 0x64, 0x76, 0x1E, 0xB9, 0x1E, 0x1B, 0xC6],
            ],
        )
        .expect_register_read(
            0b0000_0000,
            0b0000_1001,
            0xD5,
            0x60,
            [
                &[0xA2, 0x62, 0x05, 0x1F, 0xC9, 0x20, 0xEE, 0x94],
                &[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA],
            ],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, _, 2> = LTC681X::ltc6813(bus);

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
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            [&[0x2A, 0x63, 0x8E, 0x1E, 0xEC, 0x1F, 0x11, 0x0D]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_overlap_result();
    match result.unwrap_err() {
        Error::ChecksumMismatch => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_overlap_result_transfer_error() {
    let mut bus = MockSPIDevice::new();
    bus.expect_transaction().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_overlap_result();
    match result.unwrap_err() {
        Error::BusError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_overlap_results_sdo_polling() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(2).returning(|| Ok(()));
    cs.expect_set_high().times(2).returning(|| Ok(()));

    let bus = BusMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            &[[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        .expect_register_read(
            0b0000_0000,
            0b0000_1001,
            0xD5,
            0x60,
            &[[0xDE, 0x64, 0x8F, 0x21, 0x8A, 0x21, 0x8F, 0xDA]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.read_overlap_result().unwrap();
    assert_eq!(25441, result[0][0]);
    assert_eq!(7869, result[0][1]);
    assert_eq!(25822, result[0][2]);
    assert_eq!(8591, result[0][3]);
}

#[test]
fn test_read_overlap_results_sdo_polling_cs_low_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Err(PinError::Error1));

    let bus = MockSPIBus::new();
    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.read_overlap_result();
    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_overlap_results_sdo_polling_cs_high_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Ok(()));
    cs.expect_set_high().times(1).returning(|| Err(PinError::Error1));

    let bus = BusMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0000_1000,
            0x5E,
            0x52,
            &[[0x61, 0x63, 0xBD, 0x1E, 0xE4, 0x22, 0x3F, 0x42]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.read_overlap_result();
    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_ltc6813_read_internal_device_parameters() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0001_0000,
            0xED,
            0x72,
            [&[0x12, 0x62, 0xA8, 0x62, 0x00, 0x7D, 0x31, 0x8A]],
        )
        .expect_register_read(
            0b0000_0000,
            0b0001_0010,
            0x70,
            0x24,
            [&[0x00, 0xC8, 0x00, 0x66, 0x00, 0x1B, 0xF1, 0x40]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_internal_device_parameters().unwrap();
    assert_eq!(1, result.len());

    assert_eq!(75_318_000, result[0].total_voltage);
    assert_eq!("56.31578", result[0].temperature.to_string());
    assert_eq!(3_200_000, result[0].analog_power);
    assert_eq!(5_120_000, result[0].digital_power);
}

#[test]
fn test_ltc6813_read_internal_device_parameters_temp_overflow() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0001_0000,
            0xED,
            0x72,
            [&[0x12, 0x62, 0xF1, 0xD1, 0x00, 0x7D, 0xE6, 0x12]],
        )
        .expect_register_read(
            0b0000_0000,
            0b0001_0010,
            0x70,
            0x24,
            [&[0x00, 0xC8, 0x00, 0x66, 0x00, 0x1B, 0xF1, 0x40]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_internal_device_parameters().unwrap();
    assert_eq!(1, result.len());
    assert_eq!("32767.99998", result[0].temperature.to_string());
}

#[test]
fn test_ltc6813_read_internal_device_parameters_multiple_devices() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0001_0000,
            0xED,
            0x72,
            [
                &[0x12, 0x62, 0xA8, 0x62, 0x00, 0x7D, 0x31, 0x8A],
                &[0x1A, 0x59, 0x74, 0x50, 0x60, 0x6D, 0x89, 0xD8],
            ],
        )
        .expect_register_read(
            0b0000_0000,
            0b0001_0010,
            0x70,
            0x24,
            [
                &[0x00, 0xC8, 0x00, 0x66, 0x00, 0x1B, 0xF1, 0x40],
                &[0x68, 0xBF, 0x00, 0x56, 0x00, 0x2B, 0x5A, 0xC4],
            ],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, _, 2> = LTC681X::ltc6813(bus);

    let result = monitor.read_internal_device_parameters().unwrap();
    assert_eq!(2, result.len());

    assert_eq!(75_318_000, result[0].total_voltage);
    assert_eq!("56.31578", result[0].temperature.to_string());
    assert_eq!(3_200_000, result[0].analog_power);
    assert_eq!(5_120_000, result[0].digital_power);

    assert_eq!(68_430_000, result[1].total_voltage);
    assert_eq!("-5", result[1].temperature.to_string());
    assert_eq!(2_800_000, result[1].analog_power);
    assert_eq!(4_900_000, result[1].digital_power);
}

#[test]
fn test_read_internal_device_parameters_pec_error() {
    let bus = DeviceMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0001_0000,
            0xED,
            0x72,
            [&[0x2A, 0x63, 0x8E, 0x1E, 0xEC, 0x1F, 0x11, 0x0D]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_internal_device_parameters();
    match result.unwrap_err() {
        Error::ChecksumMismatch => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_internal_device_parameters_transfer_error() {
    let mut bus = MockSPIDevice::new();
    bus.expect_transaction().times(1).returning(move |_| Err(BusError::Error1));

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::ltc6813(bus);

    let result = monitor.read_internal_device_parameters();
    match result.unwrap_err() {
        Error::BusError(_) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_internal_device_parameters_sdo_polling() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(2).returning(|| Ok(()));
    cs.expect_set_high().times(2).returning(|| Ok(()));

    let bus = BusMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0001_0000,
            0xED,
            0x72,
            &[[0x12, 0x62, 0xA8, 0x62, 0x00, 0x7D, 0x31, 0x8A]],
        )
        .expect_register_read(
            0b0000_0000,
            0b0001_0010,
            0x70,
            0x24,
            &[[0x00, 0xC8, 0x00, 0x66, 0x00, 0x1B, 0xF1, 0x40]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.read_internal_device_parameters().unwrap();
    assert_eq!(1, result.len());

    assert_eq!(75_318_000, result[0].total_voltage);
    assert_eq!("56.31578", result[0].temperature.to_string());
    assert_eq!(3_200_000, result[0].analog_power);
    assert_eq!(5_120_000, result[0].digital_power);
}

#[test]
fn test_read_internal_device_parameters_sdo_polling_cs_low_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Err(PinError::Error1));

    let bus = MockSPIBus::new();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.read_internal_device_parameters();
    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_read_internal_device_parameters_sdo_polling_cs_high_error() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(|| Ok(()));
    cs.expect_set_high().times(1).returning(|| Err(PinError::Error1));

    let bus = BusMockBuilder::new()
        .expect_register_read(
            0b0000_0000,
            0b0001_0000,
            0xED,
            0x72,
            &[[0x12, 0x62, 0xA8, 0x62, 0x00, 0x7D, 0x31, 0x8A]],
        )
        .into_mock();

    let mut monitor: LTC681X<_, _, LTC6813, 1> = LTC681X::enable_sdo_polling(bus, cs);

    let result = monitor.read_internal_device_parameters();
    match result.unwrap_err() {
        Error::BusError(crate::spi::Error::CSError(PinError::Error1)) => {}
        _ => panic!("Unexpected error type"),
    }
}
