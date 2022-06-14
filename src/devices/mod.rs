use crate::device::*;
use crate::unit::*;

#[derive(Default)]
pub struct SocketM1 {
    mac: [u8; 6],
    wake: bool,
    power_provider_unit: SocketM1PowerProviderUnit,
}

#[derive(Default)]
pub struct TSensorM1 {
    mac: [u8; 6],
    sensor_unit: TSensorM1SensorUnit,
}

#[derive(Default)]
struct SocketM1PowerProviderUnit {
    provided_power: f32,
}

#[derive(Default)]
struct TSensorM1SensorUnit {
    value: f32,
}

impl<'a> Unit<'a> for SocketM1PowerProviderUnit
where
    SocketM1PowerProviderUnit: PowerProviderUnit,
{
    fn id(&self) -> &'static str {
        "SocketM1PowerProviderUnit"
    }

    fn as_trait_ref(&self) -> UnitTypeRef {
        UnitTypeRef::PowerProvider(self)
    }

    fn as_trait_mut(&mut self) -> UnitTypeMut {
        UnitTypeMut::PowerProvider(self)
    }
}

impl PowerProviderUnit for SocketM1PowerProviderUnit {
    fn measure(&self) -> Measure {
        Measure::Watt
    }

    fn provided_power(&self) -> f32 {
        self.provided_power
    }
}

impl SocketM1 {
    pub fn new() -> Self {
        Self {
            mac: rand::random::<[u8; 6]>(),
            power_provider_unit: SocketM1PowerProviderUnit { provided_power: 0. },
            wake: false,
        }
    }
}

impl Device for SocketM1
where
    Self: DeviceDescription + WakeableDevice,
{
    fn mac(&self) -> &[u8; 6] {
        &self.mac
    }

    fn id(&self) -> &'static str {
        "SocketM1"
    }

    fn as_trait_ref(&self, device_trait: DeviceTrait) -> Option<DeviceTraitRef> {
        match device_trait {
            DeviceTrait::Description => Some(DeviceTraitRef::Description(self)),
            DeviceTrait::Wakeable => Some(DeviceTraitRef::Wakeable(self)),
        }
    }

    fn as_trait_mut(&mut self, device_trait: DeviceTrait) -> Option<DeviceTraitMut> {
        match device_trait {
            DeviceTrait::Description => Some(DeviceTraitMut::Description(self)),
            DeviceTrait::Wakeable => Some(DeviceTraitMut::Wakeable(self)),
        }
    }

    fn units(&self) -> Vec<&dyn Unit> {
        vec![&self.power_provider_unit]
    }
}

impl DeviceDescription for SocketM1 {
    fn device_description(&self) -> &'static str {
        "Smart socket model 1"
    }
}

impl WakeableDevice for SocketM1 {
    fn is_awake(&self) -> bool {
        self.wake
    }

    fn wake(&mut self) {
        self.wake = true
    }

    fn sleep(&mut self) {
        self.wake = false
    }
}

impl<'a> Unit<'a> for TSensorM1SensorUnit
where
    TSensorM1SensorUnit: FloatSensorUnit,
{
    fn id(&self) -> &'static str {
        "TSensorM1SensorUnit"
    }

    fn as_trait_ref(&self) -> UnitTypeRef {
        UnitTypeRef::FloatSensor(self)
    }

    fn as_trait_mut(&mut self) -> UnitTypeMut {
        UnitTypeMut::FloatSensor(self)
    }
}

impl FloatSensorUnit for TSensorM1SensorUnit {
    fn measure(&self) -> Measure {
        Measure::Celsius
    }

    fn sensor_value(&self) -> f32 {
        self.value
    }
}

impl TSensorM1 {
    pub fn new() -> Self {
        Self {
            mac: rand::random::<[u8; 6]>(),
            sensor_unit: TSensorM1SensorUnit { value: 22. },
        }
    }
}

impl Device for TSensorM1 {
    fn mac(&self) -> &[u8; 6] {
        &self.mac
    }

    fn id(&self) -> &'static str {
        "TSensorM1"
    }

    fn units(&self) -> Vec<&dyn Unit> {
        vec![&self.sensor_unit]
    }
}

mod test {
    #[test]
    pub fn device_trait_cast() {
        use super::*;

        let socket: &dyn Device = &SocketM1::default();
        let sensor: &dyn Device = &TSensorM1::default();

        assert!(matches!(
            socket.as_trait_ref(DeviceTrait::Description),
            Some(_)
        ));
        assert!(matches!(
            socket.as_trait_ref(DeviceTrait::Wakeable),
            Some(_)
        ));

        assert!(matches!(
            sensor.as_trait_ref(DeviceTrait::Description),
            None
        ));
        assert!(matches!(sensor.as_trait_ref(DeviceTrait::Wakeable), None));
    }
}
