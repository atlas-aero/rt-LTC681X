use crate::mocks::{BusError, MockPin, MockSPIBus, PinError};
use crate::spi::{Error, LatchingSpiDevice};
use embedded_hal::spi::{Operation, SpiDevice};
use mockall::Sequence;

#[test]
fn test_read() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_read().times(1).returning(move |buffer| {
        assert_eq!(4, buffer.len());
        buffer.copy_from_slice(&[0x4, 0x5, 0x1, 0xf]);
        Ok(())
    });

    let mut device = LatchingSpiDevice::new(bus, cs);

    let mut buffer = [0x0; 4];
    device.transaction(&mut [Operation::Read(&mut buffer)]).unwrap();

    assert_eq!([0x4, 0x5, 0x1, 0xf], buffer);
}

#[test]
fn test_write() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_write().times(1).returning(move |buffer| {
        assert_eq!([0x7, 0x4, 0x2, 0x1], buffer);
        Ok(())
    });

    let mut device = LatchingSpiDevice::new(bus, cs);

    let data = [0x7, 0x4, 0x2, 0x1];
    device.transaction(&mut [Operation::Write(&data)]).unwrap();
}

#[test]
fn test_transfer() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer().times(1).returning(move |receive, send| {
        assert_eq!([0x7, 0x4, 0x2, 0x1], send);
        receive.copy_from_slice(&[0x4, 0x5, 0x1, 0xf]);
        Ok(())
    });

    let mut device = LatchingSpiDevice::new(bus, cs);

    let tx = [0x7, 0x4, 0x2, 0x1];
    let mut rx = [0x0; 4];
    device.transaction(&mut [Operation::Transfer(&mut rx, &tx)]).unwrap();
    assert_eq!([0x4, 0x5, 0x1, 0xf], rx);
}

#[test]
fn test_transfer_in_place() {
    let mut cs = MockPin::new();
    cs.expect_set_low().times(1).returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_transfer_in_place().times(1).returning(move |buffer| {
        assert_eq!([0x7, 0x4, 0x2, 0x1], buffer);
        buffer.copy_from_slice(&[0x4, 0x5, 0x1, 0xf]);
        Ok(())
    });

    let mut device = LatchingSpiDevice::new(bus, cs);

    let mut buffer = [0x7, 0x4, 0x2, 0x1];
    device.transaction(&mut [Operation::TransferInPlace(&mut buffer)]).unwrap();
    assert_eq!([0x4, 0x5, 0x1, 0xf], buffer);
}

#[test]
fn test_bus_error() {
    let mut buffers = ([0x0; 4], [0x0; 4], [0x0; 4], [0x0; 4], [0x0; 4]);
    let operations = [
        Operation::Read(&mut buffers.0),
        Operation::Write(&buffers.1),
        Operation::TransferInPlace(&mut buffers.2),
        Operation::Transfer(&mut buffers.3, &buffers.4),
    ];

    let mut bus = MockSPIBus::new();
    bus.expect_read().times(1).returning(move |_| Err(BusError::Error1));
    bus.expect_write().times(1).returning(move |_| Err(BusError::Error1));
    bus.expect_transfer().times(1).returning(move |_, _| Err(BusError::Error1));
    bus.expect_transfer_in_place()
        .times(1)
        .returning(move |_| Err(BusError::Error1));

    let mut sequence = Sequence::new();
    let mut cs = MockPin::new();

    for _ in 0..4 {
        cs.expect_set_low()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(move || Ok(()));
        cs.expect_set_high()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(move || Ok(()));
    }

    let mut device = LatchingSpiDevice::new(bus, cs);

    for operation in operations {
        let result = device.transaction(&mut [operation]);
        match result.unwrap_err() {
            Error::BusError(BusError::Error1) => {}
            _ => panic!("Unexpected error type"),
        }
    }
}

#[test]
fn test_cs_error() {
    let mut buffers = ([0x0; 4], [0x0; 4]);
    let operations = [Operation::Read(&mut buffers.0), Operation::Write(&buffers.1)];

    let mut cs = MockPin::new();
    cs.expect_set_low().times(2).returning(move || Err(PinError::Error1));

    let mut device = LatchingSpiDevice::new(MockSPIBus::new(), cs);

    for operation in operations {
        let result = device.transaction(&mut [operation]);

        match result.unwrap_err() {
            Error::CSError(PinError::Error1) => {}
            _ => panic!("Unexpected error type"),
        }
    }
}

#[test]
fn test_release_cs() {
    let mut sequence = Sequence::new();
    let mut cs = MockPin::new();
    cs.expect_set_low()
        .times(1)
        .in_sequence(&mut sequence)
        .returning(move || Ok(()));
    cs.expect_set_high()
        .times(1)
        .in_sequence(&mut sequence)
        .returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_write().times(1).returning(move |_| Ok(()));

    let mut device = LatchingSpiDevice::new(bus, cs);

    device.transaction(&mut [Operation::Write(&[0x0; 4])]).unwrap();
    device.release_cs().unwrap();
}

#[test]
fn test_release_cs_error() {
    let mut sequence = Sequence::new();
    let mut cs = MockPin::new();
    cs.expect_set_low()
        .times(1)
        .in_sequence(&mut sequence)
        .returning(move || Ok(()));
    cs.expect_set_high()
        .times(1)
        .in_sequence(&mut sequence)
        .returning(move || Err(PinError::Error1));

    let mut bus = MockSPIBus::new();
    bus.expect_write().times(1).returning(move |_| Ok(()));

    let mut device = LatchingSpiDevice::new(bus, cs);

    device.transaction(&mut [Operation::Write(&[0x0; 4])]).unwrap();
    let result = device.release_cs();

    match result.unwrap_err() {
        Error::CSError(PinError::Error1) => {}
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_cs_pin_state_cached() {
    let mut sequence = Sequence::new();
    let mut cs = MockPin::new();
    cs.expect_set_low()
        .times(1)
        .in_sequence(&mut sequence)
        .returning(move || Ok(()));
    cs.expect_set_high()
        .times(1)
        .in_sequence(&mut sequence)
        .returning(move || Ok(()));
    cs.expect_set_low()
        .times(1)
        .in_sequence(&mut sequence)
        .returning(move || Ok(()));

    let mut bus = MockSPIBus::new();
    bus.expect_write().times(3).returning(move |_| Ok(()));

    let mut device = LatchingSpiDevice::new(bus, cs);

    // CS pin is set low
    device.transaction(&mut [Operation::Write(&[0x0])]).unwrap();

    // CS pin is already low, so no GPIO interaction
    device.transaction(&mut [Operation::Write(&[0x0])]).unwrap();

    // CS pin is set high
    device.release_cs().unwrap();

    // CS pin is set low again
    device.transaction(&mut [Operation::Write(&[0x0])]).unwrap();
}
