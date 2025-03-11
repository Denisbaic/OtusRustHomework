use crate::repository::house_repository::{GetAllError, Record, Repo};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize, Clone)]
pub struct HouseDto {
    pub id: u64,
    pub name: String,
}

impl From<Record> for HouseDto {
    fn from(r: Record) -> Self {
        HouseDto {
            id: r.house.id.into(),
            name: r.house.name,
        }
    }
}

#[derive(Debug)]
pub struct Response {
    pub houses: Vec<HouseDto>,
}

/// Read all areas of life usecase interactor
pub struct ReadAllHouses<'r, R> {
    repo: &'r R,
}

impl<'r, R> ReadAllHouses<'r, R> {
    pub const fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Repo(#[from] GetAllError),
}

impl<R> ReadAllHouses<'_, R>
where
    R: Repo,
{
    pub async fn exec(&self) -> Result<Response, Error> {
        log::debug!("Read all houses");
        let houses = self
            .repo
            .list_houses()
            .await?
            .into_iter()
            .map(HouseDto::from)
            .collect();
        Ok(Response { houses })
    }
}
