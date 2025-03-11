use crate::repository::{device_repository::GetAllError, device_repository::Repo};
use domain::entities::{
    device::{Device, DeviceType},
    ids::RoomId as Id,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    /// The ID of the room.
    pub room_id: Id,
}

#[derive(Debug, Serialize, Clone)]
pub struct DeviceDto {
    pub room_id: u64,
    pub id: u64,
    pub name: String,
    pub device_type: DeviceType,
}

impl From<Device> for DeviceDto {
    fn from(r: Device) -> Self {
        Self {
            room_id: r.room_id.into(),
            id: r.id.into(),
            name: r.name,
            device_type: r.device_type,
        }
    }
}

#[derive(Debug)]
pub struct Response {
    /// The list of rooms in the house.
    pub devices: Vec<DeviceDto>,
}

/// Read all areas of life usecase interactor
pub struct ReadAllDevicesInRoom<'r, R> {
    repo: &'r R,
}

impl<'r, R> ReadAllDevicesInRoom<'r, R> {
    pub const fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Repo(#[from] GetAllError),
}

impl<R> ReadAllDevicesInRoom<'_, R>
where
    R: Repo,
{
    pub async fn exec(&self, request: Request) -> Result<Response, Error> {
        log::debug!("read all devices in room");

        let devices = self
            .repo
            .get_devices_in_room(request.room_id)
            .await?
            .into_iter()
            .map(|rec| rec.device)
            .map(DeviceDto::from)
            .collect();

        Ok(Response { devices })
    }
}
