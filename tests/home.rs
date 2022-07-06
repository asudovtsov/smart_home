use smart_home::devices::*;
use smart_home::home::*;

#[test]
fn room_name() {
    let mut home = Home::new();

    home.add_room("Kitchen".into(), Room::new()).expect("");
    home.add_room("Hall".into(), Room::new()).expect("");

    let result = home.add_room("Kitchen".into(), Room::new());
    assert!(matches!(result, Err(RoomAddError::RoomNameOccupied {})));
}

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
        Err(DeviceAddError::DeviceNameOccupied {})
    ));

    let result1 = room.add_device("".into(), Box::new(SocketM1::new()));
    assert!(matches!(result1, Err(DeviceAddError::DeviceNameIsEmpty {})));

    assert_eq!(room.device_count(), 3);
}
