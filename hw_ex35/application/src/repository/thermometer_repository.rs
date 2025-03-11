use async_trait::async_trait;
use domain::entities::{device::ThermometerInfo, ids::DeviceId};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Record {
    pub thermometer_info: ThermometerInfo,
}

#[derive(Debug, Error, Clone)]
pub enum GetError {
    #[error("ThermometerRepository connection error")]
    Connection,
    #[error("Thermometer not found")]
    NotFound,
}

#[derive(Debug, Error, Clone)]
pub enum SaveError {
    #[error("ThermometerRepository connection error")]
    Connection,
}

#[async_trait]
pub trait Repo: Send + Sync {
    async fn get_thermometer_info(&self, device_id: DeviceId) -> Result<Record, GetError>;
    async fn save_thermometer_info(&self, record: Record) -> Result<(), SaveError>;
}
