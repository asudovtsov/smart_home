use smart_home::device::*;
use smart_home::devices::*;
use smart_home::home::*;

fn main() {
    let mut home = Home::new();

    let mut hall = Room::new();
    let mut livingroom = Room::new();
    let mut kitchen = Room::new();
    let mut bedroom = Room::new();
    let mut bathroom = Room::new();

    hall.add_device("Socket".into(), Box::new(SocketM1::new()))
        .expect("");
    hall.add_device("Thermo sensor".into(), Box::new(TSensorM1::new()))
        .expect("");

    let mut livingroom_socket = Box::new(SocketM1::new());
    livingroom_socket.wake();

    livingroom
        .add_device("Socket".into(), livingroom_socket)
        .expect("");
    kitchen
        .add_device("Socket".into(), Box::new(SocketM1::new()))
        .expect("");
    bedroom
        .add_device("Socket".into(), Box::new(SocketM1::new()))
        .expect("");
    bathroom
        .add_device("Socket".into(), Box::new(SocketM1::new()))
        .expect("");

    assert_eq!(hall.device_count(), 2);
    assert_eq!(livingroom.device_count(), 1);
    assert_eq!(kitchen.device_count(), 1);
    assert_eq!(bedroom.device_count(), 1);
    assert_eq!(bathroom.device_count(), 1);

    home.add_room("Hall".into(), hall).expect("");
    home.add_room("Living room".into(), livingroom).expect("");
    home.add_room("Kitchen".into(), kitchen).expect("");
    home.add_room("Bedroom".into(), bedroom).expect("");
    home.add_room("Bathroom".into(), bathroom).expect("");

    assert_eq!(home.room_count(), 5);

    home.print_state();
}
