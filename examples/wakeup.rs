use smart_home::device::*;
use smart_home::devices::*;

fn wakeup(device: &mut dyn Device) -> bool {
    match device.as_trait_mut(DeviceTrait::Wakeable) {
        Some(DeviceTraitMut::Wakeable(d)) => {
            d.wake();
            true
        }
        _ => false,
    }
}

fn main() {
    let mut devices: Vec<Box<dyn Device>> = vec![
        Box::new(SocketM1::new()),
        Box::new(TSensorM1::new()),
        Box::new(SocketM1::new()),
    ];

    for (index, device) in devices.iter_mut().enumerate() {
        if wakeup(device.as_mut()) {
            println!("Device at index {} is awaken!", index)
        } else {
            println!("Device at index {} is never sleeps", index)
        }
    }
}
