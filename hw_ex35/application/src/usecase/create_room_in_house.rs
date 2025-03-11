use std::collections::HashSet;

use crate::{
    identifier::{NewId, NewIdError},
    repository::room_repository::{GetAllError, Repo, SaveError},
};
use domain::entities::{
    ids::{DeviceId, HouseId, RoomId},
    room::Room,
};
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub house_id: HouseId,
    pub name: String,
    pub devices_ids: HashSet<DeviceId>,
}

#[derive(Debug)]
pub struct Response;

/// Read all areas of life usecase interactor
pub struct CreateRoomInHouse<'r, 'i, R, I> {
    repo: &'r R,
    new_id_generator: &'i I,
}

impl<'r, 'i, R, I> CreateRoomInHouse<'r, 'i, R, I> {
    pub const fn new(repo: &'r R, new_id_generator: &'i I) -> Self {
        Self {
            repo,
            new_id_generator,
        }
    }
}

#[derive(Debug, Error)]
pub enum RepoError {
    #[error(transparent)]
    GetAll(#[from] GetAllError),
    #[error(transparent)]
    Save(#[from] SaveError),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Repo(#[from] RepoError),
    #[error("Room with same name already exists")]
    RoomWithSameNameExists,
    #[error(transparent)]
    Identifier(#[from] NewIdError),
}

impl<R, I> CreateRoomInHouse<'_, '_, R, I>
where
    R: Repo,
    I: NewId<RoomId>,
{
    pub async fn exec(&self, request: Request) -> Result<Response, Error> {
        log::debug!("Create room in house");

        let rooms_in_house = self
            .repo
            .get_rooms(request.house_id)
            .await
            .map_err(RepoError::GetAll)?;

        if rooms_in_house
            .iter()
            .any(|record| record.room.name == request.name)
        {
            return Err(Error::RoomWithSameNameExists);
        }

        let generated_room_id = self.new_id_generator.new_id().await?;
        let room = Room {
            id: generated_room_id,
            house_id: request.house_id,
            name: request.name,
            devices_ids: request.devices_ids,
        };
        let record = crate::repository::room_repository::Record { room };

        self.repo.save_room(record).await.map_err(RepoError::Save)?;
        Ok(Response)
    }
}
