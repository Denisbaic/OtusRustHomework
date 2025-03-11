pub mod device;
pub mod reporter;
pub mod smart_tools;
pub mod temperature;

use std::sync::{Arc, RwLock};

use crate::device::Device;
use reporter::Reporter;

pub struct Room {
    name: String,
    devices: Vec<Arc<RwLock<Box<dyn Device>>>>,
}

impl Room {
    pub fn new(name: String, devices: Vec<Arc<RwLock<Box<dyn Device>>>>) -> Self {
        Self { name, devices }
    }

    /// Add a new device to the room
    /// If the device already exists, it will not be added
    pub fn add_unique_device(&mut self, device: impl Device + 'static) -> Option<usize> {
        if self.contains_device(device.get_device_name()) {
            return None;
        }
        self.devices.push(Arc::new(RwLock::new(Box::new(device))));
        Some(self.devices.len() - 1)
    }

    pub fn remove_device(&mut self, device_name: &str) -> Option<Arc<RwLock<Box<(dyn Device)>>>> {
        let remove_pos = self
            .devices
            .iter()
            .position(|device| device.read().unwrap().get_device_name() == device_name);
        Some(self.devices.swap_remove(remove_pos?))
    }

    pub fn contains_device(&self, device_name: &str) -> bool {
        self.devices
            .iter()
            .any(|device| device.read().unwrap().get_device_name() == device_name)
    }

    pub fn get_device(&self, device_name: &str) -> Option<Arc<RwLock<Box<dyn Device>>>> {
        for device in self.devices.iter() {
            let device_ref = device.read().unwrap();
            if device_ref.get_device_name() == device_name {
                return Some(device.clone());
            }
        }
        None
    }

    pub fn get_devices(&self) -> Vec<Arc<RwLock<Box<dyn Device>>>> {
        self.devices.clone()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct SmartHouse {
    rooms: Vec<Room>,
}

impl SmartHouse {
    pub fn new(rooms: Vec<Room>) -> Self {
        Self { rooms }
    }

    pub fn set_rooms(&mut self, rooms: Vec<Room>) {
        self.rooms = rooms
    }

    /// Add a new room to the smart house
    /// If the room already exists, it will not be added
    pub fn add_unique_room(&mut self, room: Room) -> Option<usize> {
        if self.contains(&room.name) {
            return None;
        }
        self.rooms.push(room);
        Some(self.rooms.len() - 1)
    }

    pub fn remove_room(&mut self, room_name: &str) -> Option<Room> {
        let remove_pos = self.rooms.iter().position(|room| room.name == room_name);
        Some(self.rooms.swap_remove(remove_pos?))
    }

    pub fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    pub fn get_room_mut(&mut self, room_name: &str) -> Option<&mut Room> {
        self.rooms.iter_mut().find(|room| room.name == room_name)
    }

    pub fn get_room(&self, room_name: &str) -> Option<&Room> {
        self.rooms.iter().find(|room| room.name == room_name)
    }

    pub fn contains(&self, room_name: &str) -> bool {
        self.rooms.iter().any(|room| room.name == room_name)
    }

    pub fn devices(&self) -> Vec<Arc<RwLock<Box<dyn Device>>>> {
        self.rooms
            .iter()
            .flat_map(|room| room.devices.iter().cloned())
            .collect()
    }

    pub fn create_report_by_devices(
        &self,
        room_name_device_name: Vec<(&str, &str)>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let title = "===============Smart House Report===============";
        let mut content = String::new();
        for (room_name, device_name) in room_name_device_name {
            match self.get_room(room_name) {
                Some(room) => match room.get_device(device_name) {
                    Some(device) => match device.read().unwrap().create_report() {
                        Ok(report) => {
                            content.push_str(report.as_str());
                            content.push('\n');
                        }

                        Err(err) => return Err(format!("Error: {}", err).into()),
                    },
                    None => return Err(format!("Device {} not found", device_name).into()),
                },
                None => return Err(format!("Room {} not found", room_name).into()),
            }
        }
        let end = "===============Smart House Report end===========";
        Ok(format!("{title}\n{content}{end}\n"))
    }
}

impl Reporter for SmartHouse {
    fn create_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        let title = "===============Smart House Report===============";

        let mut content = String::new();

        for room in self.get_rooms() {
            let report_title = format!("======={}======", room.name);
            let mut room_content = String::new();

            for device in &room.devices {
                let report = match device.read().unwrap().create_report() {
                    Ok(report) => report,
                    Err(err) => return Err(format!("Error: {}", err).into()),
                };
                room_content.push_str(&format!("\n {}", report));
            }

            let end = "=".repeat(report_title.chars().count());

            content.push_str(&format!("\n{report_title}\n {room_content} {end}\n"));
        }

        let end = "===============Smart House Report end===========";

        Ok(format!("{title}\n{content}{end}\n"))
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn test_smart_house_creation() {
        let house = SmartHouse::new(vec![]);
        assert_eq!(house.get_rooms().len(), 0);
    }

    #[test]
    fn test_add_unique_room() {
        let mut house = SmartHouse::new(vec![]);
        assert_eq!(
            house.add_unique_room(Room::new("Room 1".to_string(), vec![])),
            Some(0)
        );
        assert_eq!(
            house.add_unique_room(Room::new("Room 1".to_string(), vec![])),
            None
        );
    }

    #[test]
    fn test_remove_room() {
        let mut house = SmartHouse::new(vec![Room::new("Room 1".to_string(), vec![])]);
        let removed_room = house.remove_room("Room 1");
        assert!(removed_room.is_some());
        let removed_room = removed_room.unwrap();
        assert_eq!(removed_room.name, "Room 1");
    }

    #[test]
    fn test_get_room() {
        let house = SmartHouse::new(vec![Room::new("Room 1".to_string(), vec![])]);
        let room = house.get_room("Room 1");
        assert!(room.is_some());
        let room = room.unwrap();
        assert_eq!(room.name, "Room 1");
    }

    #[test]
    fn test_get_room_mut() {
        let mut house = SmartHouse::new(vec![Room::new("Room 1".to_string(), vec![])]);
        let room = house.get_room_mut("Room 1");
        assert!(room.is_some());
        let room = room.unwrap();
        assert_eq!(room.name, "Room 1");
    }

    #[test]
    fn test_contains() {
        let house = SmartHouse::new(vec![Room::new("Room 1".to_string(), vec![])]);
        assert!(house.contains("Room 1"));
    }

    struct StubDevice {
        name: &'static str,
    }

    impl Reporter for StubDevice {
        fn create_report(&self) -> Result<String, Box<dyn std::error::Error>> {
            Ok("Dummy device".to_string())
        }
    }
    impl Device for StubDevice {
        fn turn_on(&mut self) {
            todo!()
        }

        fn turn_off(&mut self) {
            todo!()
        }

        fn is_on(&self) -> bool {
            true
        }

        fn is_off(&self) -> bool {
            todo!()
        }

        fn get_device_name(&self) -> &str {
            self.name
        }
    }

    #[test]
    fn test_add_devices() {
        let mut house = SmartHouse::new(vec![Room::new("Room 1".to_string(), vec![])]);
        let devices_before_add_len = house.devices().len();
        assert_eq!(devices_before_add_len, 0);

        let room_opt = house.get_room_mut("Room 1");
        assert!(room_opt.is_some());
        if let Some(room) = room_opt {
            room.add_unique_device(StubDevice {
                name: "Dummy device",
            });
        }
        let devices_after_add_len = house.devices().len();
        assert_ne!(devices_after_add_len, devices_before_add_len);

        let room_opt = house.get_room_mut("Room 1");
        if let Some(room) = room_opt {
            room.add_unique_device(StubDevice {
                name: "Dummy device",
            });
        }
        let devices_after_add_len = house.devices().len();
        assert_eq!(devices_after_add_len, 1);
    }

    #[test]
    fn test_create_report_by_devices() {
        let mut house = SmartHouse::new(vec![Room::new("Room 1".to_string(), vec![])]);
        let room_opt = house.get_room_mut("Room 1");
        assert!(room_opt.is_some());
        if let Some(room) = room_opt {
            room.add_unique_device(StubDevice {
                name: "Dummy device",
            });
            room.add_unique_device(StubDevice {
                name: "Dummy device2",
            });
        }
        let room_name_device_name = vec![("Room 1", "Dummy device"), ("Room 1", "Dummy device2")];
        let report = house.create_report_by_devices(room_name_device_name);
        assert!(report.is_ok());
    }

    #[test]
    fn test_create_report_by_devices_error() {
        let mut house = SmartHouse::new(vec![Room::new("Room 1".to_string(), vec![])]);
        let room_opt = house.get_room_mut("Room 1");
        assert!(room_opt.is_some());
        if let Some(room) = room_opt {
            room.add_unique_device(StubDevice {
                name: "Dummy device",
            });
            room.add_unique_device(StubDevice {
                name: "Dummy device2",
            });
        }
        let room_name_device_name = vec![("Room 1", "Dummy device"), ("Room 2", "Dummy device2")];
        let report = house.create_report_by_devices(room_name_device_name);
        assert!(report.is_err());
    }

    #[test]
    fn test_create_report_by_devices_and_dynamic_rooms() {
        let mut house = SmartHouse::new(vec![]);
        house.add_unique_room(Room::new("Room 1".to_string(), vec![]));
        let room_opt = house.get_room_mut("Room 1");
        assert!(room_opt.is_some());
        if let Some(room) = room_opt {
            room.add_unique_device(StubDevice {
                name: "Dummy device",
            });
            room.add_unique_device(StubDevice {
                name: "Dummy device2",
            });
        }
        let room_name_device_name = vec![("Room 1", "Dummy device"), ("Room 1", "Dummy device2")];
        let report = house.create_report_by_devices(room_name_device_name);
        assert!(report.is_ok());
    }

    #[test]
    fn test_create_report_by_devices_and_dynamic_rooms2() {
        let mut house = SmartHouse::new(vec![]);
        house.add_unique_room(Room::new("Room 1".to_string(), vec![]));
        let room_opt = house.get_room_mut("Room 1");
        assert!(room_opt.is_some());
        if let Some(room) = room_opt {
            room.add_unique_device(StubDevice {
                name: "Dummy device",
            });
            room.add_unique_device(StubDevice {
                name: "Dummy device2",
            });
        }
        house.add_unique_room(Room::new("Room 2".to_string(), vec![]));
        let room_opt = house.get_room_mut("Room 2");
        assert!(room_opt.is_some());
        if let Some(room) = room_opt {
            room.add_unique_device(StubDevice {
                name: "Dummy device2",
            });
        }
        house.add_unique_room(Room::new("Room 2".to_string(), vec![]));
        house.remove_room("Room 3");
        let room_name_device_name = vec![("Room 1", "Dummy device"), ("Room 2", "Dummy device2")];
        let report = house.create_report_by_devices(room_name_device_name);
        assert!(report.is_ok());
    }

    #[test]
    fn test_create_report() {
        let house = SmartHouse::new(vec![Room::new("Room 1".to_string(), vec![])]);
        let report = house.create_report();
        assert!(report.is_ok());
    }
}
