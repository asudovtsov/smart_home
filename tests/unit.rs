use smart_home::device::*;
use smart_home::devices::*;
use smart_home::unit::*;

#[test]
fn measure() {
    let devices: Vec<Box<dyn Device>> = vec![Box::new(SocketM1::new()), Box::new(TSensorM1::new())];

    for ref device in devices {
        for unit in device.units() {
            match unit.as_trait_ref() {
                UnitTypeRef::FloatSensor(u) => {
                    assert!(matches!(
                        u.measure(),
                        Measure::Celsius | Measure::Fahrenheit | Measure::Kelvin
                    ));
                }
                UnitTypeRef::PowerProvider(u) => {
                    assert!(matches!(u.measure(), Measure::Watt | Measure::Erg));
                }
                _ => {}
            }
        }
    }
}

#[test]
fn value_bound() {
    let devices: Vec<Box<dyn Device>> = vec![Box::new(SocketM1::new()), Box::new(TSensorM1::new())];

    for ref device in devices {
        for unit in device.units() {
            match unit.as_trait_ref() {
                UnitTypeRef::FloatSensor(u) => {
                    assert!(u.sensor_value().is_finite());
                    assert!(u.sensor_value() >= 0.);
                }
                UnitTypeRef::PowerProvider(u) => {
                    assert!(u.provided_power().is_finite());
                    assert!(u.provided_power() >= 0.);
                }
                _ => {}
            }
        }
    }
}

#[test]
fn id() {
    let devices: Vec<Box<dyn Device>> = vec![Box::new(SocketM1::new()), Box::new(TSensorM1::new())];

    for ref device in devices {
        for unit in device.units() {
            assert!(!unit.id().is_empty());
        }
    }
}
