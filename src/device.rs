use crate::unit::*;

#[non_exhaustive]
pub enum DeviceTrait {
    Description,
    Wakeable,
}

#[non_exhaustive]
pub enum DeviceTraitRef<'a> {
    Description(&'a dyn DeviceDescription),
    Wakeable(&'a dyn WakeableDevice),
}

#[non_exhaustive]
pub enum DeviceTraitMut<'a> {
    Description(&'a mut dyn DeviceDescription),
    Wakeable(&'a mut dyn WakeableDevice),
}

pub trait Device {
    fn mac(&self) -> &[u8; 6];
    fn id(&self) -> &'static str;

    fn as_trait_ref(&self, _: DeviceTrait) -> Option<DeviceTraitRef> {
        None
    }

    fn as_trait_mut(&mut self, _: DeviceTrait) -> Option<DeviceTraitMut> {
        None
    }

    fn units(&self) -> Vec<&dyn Unit>;
}

pub trait WakeableDevice {
    fn is_awake(&self) -> bool;
    fn wake(&mut self);
    fn sleep(&mut self);
}

pub trait DeviceDescription {
    fn device_description(&self) -> &'static str;
}
