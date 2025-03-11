use crate::repository::room_repository::{DeleteError, Repo};
use domain::entities::ids::RoomId;
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub room_id: RoomId,
}

#[derive(Debug)]
pub struct Response;

pub struct DeleteRoomInHouse<'r, R> {
    repo: &'r R,
}

impl<'r, R> DeleteRoomInHouse<'r, R> {
    pub const fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Repo(#[from] DeleteError),
}

impl<R> DeleteRoomInHouse<'_, R>
where
    R: Repo,
{
    pub async fn exec(&self, request: Request) -> Result<Response, Error> {
        log::debug!("Delete room in house");

        self.repo.delete_room(request.room_id).await?;
        Ok(Response)
    }
}
