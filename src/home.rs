use std::collections::{hash_map::Entry, HashMap};

use crate::device::*;
use crate::unit::*;

#[derive(Debug)]
pub enum AddDeviceError {
    DeviceNameIsEmpty,
    DeviceNameOccupied,
}

#[derive(Default)]
pub struct Room {
    devices: HashMap<String, Box<dyn Device>>,
}

#[derive(Default)]
pub struct Home {
    rooms: HashMap<String, Room>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn for_devices<F>(&self, f: F)
    where
        F: Fn(&String, &Box<dyn Device>),
    {
        for (name, device) in &self.devices {
            f(name, device)
        }
    }

    pub fn add_device(
        &mut self,
        name: String,
        device: Box<dyn Device>,
    ) -> Result<(), AddDeviceError> {
        if name.is_empty() {
            return Err(AddDeviceError::DeviceNameIsEmpty);
        }

        match self.devices.entry(name) {
            Entry::Occupied(_) => Err(AddDeviceError::DeviceNameOccupied),
            Entry::Vacant(e) => {
                e.insert(device);
                Ok(())
            }
        }
    }
}

impl Home {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    pub fn room_count(&self) -> usize {
        self.rooms.len()
    }

    pub fn for_rooms<F>(&self, f: F)
    where
        F: Fn(&String, &Room),
    {
        for (name, room) in &self.rooms {
            f(name, room)
        }
    }

    pub fn add_room(&mut self, name: String, room: Room) -> bool {
        if let Entry::Vacant(e) = self.rooms.entry(name) {
            e.insert(room);
            return true;
        }
        false
    }

    pub fn unit_state(unit: &dyn Unit) -> String {
        match unit.as_trait_ref() {
            UnitTraitRef::FloatSensor(u) => {
                format!("sensor value: {}", u.sensor_value())
            }
            UnitTraitRef::PowerProvider(u) => {
                format!("provided power: {}", u.provided_power())
            }
        }
    }

    pub fn device_state(device: &dyn Device) -> String {
        let mut state = String::new();
        if let Some(DeviceTraitRef::Description(d)) = device.as_trait_ref(DeviceTrait::Description)
        {
            state += format!("description: {}", d.device_description()).as_str();
        }
        if let Some(DeviceTraitRef::Wakeable(d)) = device.as_trait_ref(DeviceTrait::Wakeable) {
            if !state.is_empty() {
                state += ", "
            }
            state += format!("is awake: {}", d.is_awake()).as_str();
        }
        state.trim().into()
    }

    pub fn print_state(&self) {
        self.for_rooms(|room_name, room| {
            room.for_devices(|device_name, device| {
                let mut state = format!("Room: {room_name}, device: {device_name}");

                let device_state = Self::device_state(device.as_ref());
                if !device_state.is_empty() {
                    state += format!(", {}", device_state).as_str();
                }

                for unit in device.units() {
                    state += format!(", {}", Self::unit_state(unit)).as_str();
                }

                println!("{}", state);
            })
        });
    }
}

mod tests {
    #[test]
    fn room_name() {
        use super::*;

        let mut home = Home::new();
        home.add_room("Hall".into(), Room::new());
        home.add_room("Hall".into(), Room::new());
        home.add_room("Kitchen".into(), Room::new());
        assert_eq!(home.room_count(), 2);
    }
}
