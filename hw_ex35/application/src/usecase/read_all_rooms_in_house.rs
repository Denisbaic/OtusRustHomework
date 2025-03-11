use std::collections::HashSet;

use crate::repository::room_repository::{GetAllError, Repo};
use domain::entities::{ids::HouseId as Id, room::Room};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    /// The ID of the house.
    pub house_id: Id,
}

#[derive(Debug, Serialize, Clone)]
pub struct RoomDto {
    pub house_id: u64,
    pub id: u64,
    pub name: String,
    pub devices_ids: HashSet<u64>,
}

impl From<Room> for RoomDto {
    fn from(r: Room) -> Self {
        let house_id = r.house_id;
        let room_id = r.id;
        let devices_ids: HashSet<u64> = r.devices_ids.iter().map(|d| (*d).into()).collect();

        RoomDto {
            house_id: house_id.into(),
            id: room_id.into(),
            name: r.name,
            devices_ids,
        }
    }
}

#[derive(Debug)]
pub struct Response {
    /// The list of rooms in the house.
    pub rooms: Vec<RoomDto>,
}

/// Read all areas of life usecase interactor
pub struct ReadAllRoomsInHouse<'r, R> {
    repo: &'r R,
}

impl<'r, R> ReadAllRoomsInHouse<'r, R> {
    pub const fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Repo(#[from] GetAllError),
}

impl<R> ReadAllRoomsInHouse<'_, R>
where
    R: Repo,
{
    pub async fn exec(&self, request: Request) -> Result<Response, Error> {
        log::debug!("Read all rooms in house");
        let rooms = self
            .repo
            .get_rooms(request.house_id)
            .await?
            .into_iter()
            .map(Room::from)
            .map(RoomDto::from)
            .collect();
        Ok(Response { rooms })
    }
}
