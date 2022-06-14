use smart_home::devices::*;
use smart_home::home::*;

#[test]
fn device_name() {
    let mut room = Room::new();

    room.add_device("Socket".into(), Box::new(SocketM1::new()))
        .expect("");
    room.add_device("ThermoSensor".into(), Box::new(TSensorM1::new()))
        .expect("");
    room.add_device("My favorite device".into(), Box::new(SocketM1::new()))
        .expect("");

    let result0 = room.add_device("My favorite device".into(), Box::new(SocketM1::new()));
    assert!(matches!(
        result0,
        Err(AddDeviceError::DeviceNameOccupied {})
    ));

    let result1 = room.add_device("".into(), Box::new(SocketM1::new()));
    assert!(matches!(result1, Err(AddDeviceError::DeviceNameIsEmpty {})));

    assert_eq!(room.device_count(), 3);
}
