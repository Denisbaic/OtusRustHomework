use async_trait::async_trait;
use domain::entities::{house::House, ids::HouseId};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Record {
    pub house: House,
}

#[derive(Debug, Error)]
pub enum GetAllError {
    #[error("HouseRepository connection error")]
    Connection,
    #[error("Room not found")]
    NotFound,
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("HouseRepository connection error")]
    Connection,
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("HouseRepository connection error")]
    Connection,
}

#[async_trait]
pub trait Repo: Send + Sync {
    async fn list_houses(&self) -> Result<Vec<Record>, GetAllError>;
    async fn save_house(&self, record: Record) -> Result<(), SaveError>;
    async fn delete_house(&self, house_id: HouseId) -> Result<(), DeleteError>;
}
