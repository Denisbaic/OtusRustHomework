pub mod read_all_houses;

pub mod create_room_in_house;
pub mod delete_room_in_house;
pub mod read_all_rooms_in_house;

pub mod create_device_in_room;
pub mod delete_device_in_room;
pub mod read_all_devices_in_room;

pub mod get_report_from_devices;

/*
use domain::entities::{
    device::DeviceType,
    ids::{DeviceId, HouseId, RoomId},
};

struct ListRoomsInHouseDTO {
    house_id: HouseId,
}
struct AddRoomToHouseDTO {
    house_id: HouseId,
    name: String,
}
struct RemoveRoomFromHouseDTO {
    house_id: HouseId,
    room_id: RoomId,
}
struct GetDevicesInRoomDTO {
    room_id: RoomId,
}
struct AddDeviceToRoomDTO {
    room_id: RoomId,
    name: String,
    device_type: DeviceType,
}
struct RemoveDeviceFromRoomDTO {
    room_id: RoomId,
    device_id: DeviceId,
}

struct GetReportFromDeviceDTO {
    device_id: DeviceId,
}

// Usecase - port (первичный)
pub(crate) trait SmartHouseUsecase {
    async fn list_rooms_in_house(
        &self,
        list_rooms_in_house_dto: ListRoomsInHouseDTO,
    ) -> Result<Vec<Room>, ()>;
    async fn add_room_to_house(&self, add_room_to_house_dto: AddRoomToHouseDTO)
        -> Result<Room, ()>;
    async fn remove_room_from_house(
        &self,
        remove_room_from_house_dto: RemoveRoomFromHouseDTO,
    ) -> Result<(), ()>;
    async fn get_devices_in_room(
        &self,
        get_devices_in_room_dto: GetDevicesInRoomDTO,
    ) -> Result<Vec<Device>, ()>;
    async fn add_device_to_room(
        &self,
        add_device_to_room_dto: AddDeviceToRoomDTO,
    ) -> Result<Device, ()>;
    async fn remove_device_to_room(
        &self,
        remove_device_from_room_dto: RemoveDeviceFromRoomDTO,
    ) -> Result<Device, ()>;
    async fn get_report_from_device(
        &self,
        get_report_from_device_dto: GetReportFromDeviceDTO,
    ) -> Result<String, ()>;
}
*/
