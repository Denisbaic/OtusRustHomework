use std::collections::HashSet;

use super::ids::{DeviceId, HouseId, RoomId};

#[derive(Debug, Clone)]
pub struct Room {
    pub house_id: HouseId,
    pub id: RoomId,
    pub name: String,
    pub devices_ids: HashSet<DeviceId>,
}
