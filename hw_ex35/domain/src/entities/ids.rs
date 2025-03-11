use crate::id;

use super::{device::Device, house::House, room::Room};

pub type DeviceId = id::Id<Device>;

pub type HouseId = id::Id<House>;

pub type RoomId = id::Id<Room>;
