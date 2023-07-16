#![allow(dead_code)]

pub mod info_provider;
pub mod smart_device;

use crate::smart_house::info_provider::DeviceInfoProvider;
use crate::smart_house::smart_device::SmartDevice;
use std::collections::HashMap;
use std::fmt::Debug;

pub struct Room {
    pub devices: HashMap<String, Box<dyn SmartDevice>>,
}

impl Room {
    pub fn new() -> Self {
        Room {
            devices: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub enum SmartHouseError {
    NoSuchRoom,
    NoSuchDevice,
}

pub struct SmartHouse {
    /* данные умного дома */
    //Помещение имеет уникальное название и содержит названия нескольких устройств.
    //Устройство имеет уникальное в рамках помещения имя.
    pub name: String,
    pub rooms: HashMap<String, Room>,
}

impl SmartHouse {
    //Дом имеет название и содержит несколько помещений.
    pub fn new(name: &str) -> Self {
        SmartHouse {
            name: String::from(name),
            rooms: HashMap::new(),
        }
    }

    //Библиотека позволяет запросить список помещений в доме.
    pub fn get_rooms(&self) -> Vec<String> {
        //"список комнат"
        self.rooms.keys().cloned().collect::<Vec<String>>()
    }

    pub fn get_devices(&self, room_name: &str) -> Result<Vec<String>, SmartHouseError> {
        //"список устройств в комнате `room`"
        let room = self.rooms.get(room_name);
        match room {
            None => Err(SmartHouseError::NoSuchRoom),
            Some(_) => Ok(room
                .unwrap()
                .devices
                .keys()
                .cloned()
                .collect::<Vec<String>>()),
        }
    }

    pub fn create_report(&self, dip: &dyn DeviceInfoProvider) -> String {
        dip.device_info()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let house = SmartHouse::new("test");
        assert_eq!(house.name, "test");
        assert_eq!(house.rooms.len(), 0);
    }
}
