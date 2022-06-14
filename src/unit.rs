pub enum Measure {
    Celsius,
    Fahrenheit,
    Kelvin,
    Watt,
    Erg,
}

#[non_exhaustive]
pub enum UnitTraitRef<'a> {
    FloatSensor(&'a dyn FloatSensorUnit),
    PowerProvider(&'a dyn PowerProviderUnit),
}

#[non_exhaustive]
pub enum UnitTraitMut<'a> {
    FloatSensor(&'a mut dyn FloatSensorUnit),
    PowerProvider(&'a mut dyn PowerProviderUnit),
}

pub trait Unit<'a> {
    fn id(&self) -> &'static str;

    fn as_trait_ref(&self) -> UnitTraitRef;
    fn as_trait_mut(&mut self) -> UnitTraitMut;
}

pub trait FloatSensorUnit {
    fn measure(&self) -> Measure;
    fn sensor_value(&self) -> f32;
}

pub trait PowerProviderUnit {
    fn measure(&self) -> Measure;
    fn provided_power(&self) -> f32;
}
