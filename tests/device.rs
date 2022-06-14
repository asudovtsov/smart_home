use smart_home::device::*;
use smart_home::devices::*;

#[test]
fn description() {
    let devices: Vec<Box<dyn Device>> = vec![Box::new(SocketM1::new()), Box::new(TSensorM1::new())];

    for ref device in devices {
        if let Some(DeviceTraitRef::Description(d)) = device.as_trait_ref(DeviceTrait::Description)
        {
            assert!(!d.device_description().is_empty())
        }
    }
}