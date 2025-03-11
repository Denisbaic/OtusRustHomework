use async_trait::async_trait;
use domain::entities::{device::LightInfo, ids::DeviceId};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Record {
    pub light_info: LightInfo,
}

#[derive(Debug, Error, Clone)]
pub enum GetError {
    #[error("LightRepository connection error")]
    Connection,
    #[error("Room not found")]
    NotFound,
}

#[derive(Debug, Error, Clone)]
pub enum SaveError {
    #[error("LightRepository connection error")]
    Connection,
}

#[async_trait]
pub trait Repo: Send + Sync {
    async fn get_light_info(&self, device_id: DeviceId) -> Result<Record, GetError>;
    async fn save_light_info(&self, record: Record) -> Result<(), SaveError>;
}
