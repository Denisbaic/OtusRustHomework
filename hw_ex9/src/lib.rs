pub mod device;
pub mod reporter;
pub mod smart_tools;
pub mod temperature;

use crate::device::Device;
use reporter::Reporter;

pub struct Room {
    name: String,
    devices: Vec<Box<dyn Device>>,
}

impl Room {
    pub fn new(name: String, devices: Vec<Box<dyn Device>>) -> Self {
        Self { name, devices }
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) {
        self.devices.push(device);
    }

    pub fn get_devices(&self) -> Vec<&dyn Device> {
        self.devices.iter().map(|device| &**device).collect()
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

    pub fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    pub fn devices(&self) -> Vec<&dyn Device> {
        self.get_rooms()
            .iter()
            .flat_map(|room| room.devices.iter().map(|device| &**device))
            .collect()
    }

    pub fn create_report_by_devices(&self) -> String {
        String::new()
    }
}

impl Reporter for SmartHouse {
    fn create_report(&self) -> String {
        let title = "===============Smart House Report===============";
        let content = self.get_rooms().iter().fold(String::new(), |acc, room| {
            let report_title = format!("======={}======", room.name);
            let content = room.devices.iter().fold(String::new(), |acc, device| {
                format!("{acc}\n {}", device.create_report())
            });

            let end = "=".repeat(report_title.chars().count());

            format!("{acc}\n{report_title}\n {content} {end}\n")
        });

        let end = "===============Smart House Report end===========";

        format!("{title}\n{content}{end}\n")
    }
}
