use crate::repository::{
    self,
    device_repository::{DeleteError, GetAllError, SaveError},
};
use domain::entities::ids::DeviceId;
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub device_id: DeviceId,
}

#[derive(Debug)]
pub struct Response;

/// Read all areas of life usecase interactor
pub struct DeleteDeviceInRoom<'r, R> {
    repo: &'r R,
}

impl<'r, R> DeleteDeviceInRoom<'r, R> {
    pub const fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum RepoError {
    #[error(transparent)]
    GetAll(#[from] GetAllError),
    #[error(transparent)]
    Save(#[from] SaveError),
    #[error(transparent)]
    SaveThermometerInfo(#[from] repository::thermometer_repository::SaveError),
    #[error(transparent)]
    SaveLightInfo(#[from] repository::light_repository::SaveError),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Repo(#[from] DeleteError),
}

impl<R> DeleteDeviceInRoom<'_, R>
where
    R: repository::device_repository::Repo,
{
    pub async fn exec(&self, request: Request) -> Result<Response, Error> {
        log::debug!("Delete device in room");

        self.repo.delete_device(request.device_id).await?;

        Ok(Response)
    }
}
