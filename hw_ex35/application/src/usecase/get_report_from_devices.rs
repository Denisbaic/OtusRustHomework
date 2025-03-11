use std::collections::HashSet;

use crate::{
    identifier::NewIdError,
    repository::{
        self,
        device_repository::{GetError, Record},
    },
};

use domain::entities::{device::DeviceType, ids::DeviceId};
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub device_ids: HashSet<DeviceId>,
}

#[derive(Debug)]
pub struct Response {
    pub device_reports: Vec<String>,
}

/// Read all areas of life usecase interactor
pub struct GetReportFromDevices<'r, R> {
    repo: &'r R,
}

impl<'r, R> GetReportFromDevices<'r, R> {
    pub const fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error, Clone)]
pub enum RepoError {
    #[error(transparent)]
    Get(#[from] GetError),
    #[error(transparent)]
    GetThermometerInfo(#[from] repository::thermometer_repository::GetError),
    #[error(transparent)]
    GetLightInfo(#[from] repository::light_repository::GetError),
}

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error(transparent)]
    Repo(#[from] RepoError),
    #[error("Device with same name already exists")]
    DeviceWithSameNameExists,
    #[error("Cant get device type from str : {0}")]
    CantGetDeviceTypeFromStr(String),
    #[error(transparent)]
    Identifier(#[from] NewIdError),
}

impl<R> GetReportFromDevices<'_, R>
where
    R: repository::device_repository::Repo
        + repository::light_repository::Repo
        + repository::thermometer_repository::Repo,
{
    pub async fn exec(&self, request: Request) -> Result<Response, Error> {
        log::debug!("Get report from devices");

        let mut device_reports = Vec::new();
        for id in request.device_ids {
            let get_device_result= self.repo.get_device(id).await.map_err(RepoError::Get);

            if get_device_result.is_err() {
                let id : u64 = id.into();
                device_reports.push(format!(
                    "Id {} error {}",
                    id, get_device_result.clone().err().unwrap()
                ));
                continue;
            }

            let Record { device } = get_device_result.unwrap();

            match device.device_type {
                DeviceType::Thermometer => {
                    let repository::thermometer_repository::Record { thermometer_info } = self
                        .repo
                        .get_thermometer_info(id)
                        .await
                        .map_err(RepoError::GetThermometerInfo)?;
                    let id : u64 = id.into();
                    device_reports.push(format!(
                        " Id {}, Device: {}, Temperature: {}",
                        id, device.name, thermometer_info.temperature
                    ));
                }
                DeviceType::Light => {
                    let repository::light_repository::Record { light_info } = self
                        .repo
                        .get_light_info(id)
                        .await
                        .map_err(RepoError::GetLightInfo)?;
                    let id : u64 = id.into();
                    device_reports.push(format!(
                        "Id {}, Device: {}, Light intensity: {}",
                        id, device.name, light_info.intensity
                    ));
                }
            }
        }

        Ok(Response { device_reports })
    }
}
