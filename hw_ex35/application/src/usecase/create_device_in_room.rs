use crate::{
    identifier::{NewId, NewIdError},
    repository::{
        self,
        device_repository::{GetAllError, SaveError},
    },
};
use domain::entities::{
    device::{Device, DeviceType, LightInfo, Temperature, ThermometerInfo},
    ids::{DeviceId, RoomId},
};
use rand::Rng;
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub room_id: RoomId,
    pub device_name: String,
    pub device_type: String,
}

#[derive(Debug)]
pub struct Response;

/// Read all areas of life usecase interactor
pub struct CreateDeviceInRoom<'r, 'i, R, I> {
    repo: &'r R,
    new_id_generator: &'i I,
}

impl<'r, 'i, R, I> CreateDeviceInRoom<'r, 'i, R, I> {
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
    #[error(transparent)]
    SaveThermometerInfo(#[from] repository::thermometer_repository::SaveError),
    #[error(transparent)]
    SaveLightInfo(#[from] repository::light_repository::SaveError),
}

#[derive(Debug, Error)]
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

impl<R, I> CreateDeviceInRoom<'_, '_, R, I>
where
    R: repository::device_repository::Repo
        + repository::light_repository::Repo
        + repository::thermometer_repository::Repo,
    I: NewId<DeviceId>,
{
    pub async fn exec(&self, request: Request) -> Result<Response, Error> {
        log::debug!("Create device in room");

        let devices_in_room = self
            .repo
            .get_devices_in_room(request.room_id)
            .await
            .map_err(RepoError::GetAll)?;

        if devices_in_room
            .iter()
            .any(|record| record.device.name == request.device_name)
        {
            return Err(Error::DeviceWithSameNameExists);
        }

        let generated_device_id = self.new_id_generator.new_id().await?;
        let device_type = Self::string_to_device_type(request.device_type.clone())
            .ok_or(Error::CantGetDeviceTypeFromStr(request.device_type))?;
        let device = Device {
            room_id: request.room_id,
            device_type,
            id: generated_device_id,
            name: request.device_name,
        };
        let record = crate::repository::device_repository::Record { device };

        self.repo
            .save_device(record)
            .await
            .map_err(RepoError::Save)?;

        match device_type {
            DeviceType::Thermometer => {
                let thermometer_info = ThermometerInfo {
                    device_id: generated_device_id,
                    temperature: Temperature::new(
                        rand::rng().random_range(-100.0..100.0),
                        domain::entities::device::TemperatureMeasureUnits::Celsius,
                    ),
                };
                let record = crate::repository::thermometer_repository::Record { thermometer_info };
                self.repo
                    .save_thermometer_info(record)
                    .await
                    .map_err(RepoError::SaveThermometerInfo)?;
            }
            DeviceType::Light => {
                let light_info = LightInfo {
                    device_id: generated_device_id,
                    intensity: rand::rng().random_range(0.0..100.0),
                };
                let record = crate::repository::light_repository::Record { light_info };
                self.repo
                    .save_light_info(record)
                    .await
                    .map_err(RepoError::SaveLightInfo)?;
            }
        }
        Ok(Response)
    }

    fn string_to_device_type(device_type_str: String) -> Option<DeviceType> {
        match device_type_str.to_lowercase().as_str() {
            "light" => Some(DeviceType::Light),
            "thermometer" => Some(DeviceType::Thermometer),
            _ => None,
        }
    }
}
