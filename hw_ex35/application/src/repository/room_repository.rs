use async_trait::async_trait;
use domain::entities::{
    ids::{HouseId, RoomId},
    room::Room,
};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Record {
    pub room: Room,
}

impl From<Record> for Room {
    fn from(r: Record) -> Self {
        let Record { room } = r;
        room
    }
}

#[derive(Debug, Error)]
pub enum GetAllError {
    #[error("RoomRepository connection error")]
    Connection,
    #[error("House not found")]
    NotFound,
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("RoomRepository connection error")]
    Connection,
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("RoomRepository connection error")]
    Connection,
}

#[async_trait]
pub trait Repo: Send + Sync {
    async fn get_rooms(&self, house_id: HouseId) -> Result<Vec<Record>, GetAllError>;
    async fn save_room(&self, record: Record) -> Result<(), SaveError>;
    async fn delete_room(&self, room_id: RoomId) -> Result<(), DeleteError>;
}
