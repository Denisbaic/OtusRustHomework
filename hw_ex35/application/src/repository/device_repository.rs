use async_trait::async_trait;
use domain::entities::{
    device::Device,
    ids::{DeviceId, RoomId},
};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Record {
    pub device: Device,
}

#[derive(Debug, Error, Clone)]
pub enum GetAllError {
    #[error("DeviceRepository connection error")]
    Connection,
    #[error("Room not found")]
    NotFound,
}

#[derive(Debug, Error, Clone)]
pub enum GetError {
    #[error("DeviceRepository connection error")]
    Connection,
    #[error("Device not found")]
    NotFound,
}

#[derive(Debug, Error, Clone)]
pub enum SaveError {
    #[error("DeviceRepository connection error")]
    Connection,
}

#[derive(Debug, Error, Clone)]
pub enum DeleteError {
    #[error("DeviceRepository connection error")]
    Connection,
}

#[async_trait]
pub trait Repo: Send + Sync {
    async fn get_devices_in_room(&self, room_id: RoomId) -> Result<Vec<Record>, GetAllError>;
    async fn get_device(&self, device_id: DeviceId) -> Result<Record, GetError>;
    async fn save_device(&self, record: Record) -> Result<(), SaveError>;
    async fn delete_device(&self, device_id: DeviceId) -> Result<(), DeleteError>;
}
