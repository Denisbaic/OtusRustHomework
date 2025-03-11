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
    fn create_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        let title = "===============Smart House Report===============";

        let mut content = String::new();

        for room in self.get_rooms() {
            let report_title = format!("======={}======", room.name);
            let mut room_content = String::new();

            for device in &room.devices {
                let report = match device.create_report() {
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
