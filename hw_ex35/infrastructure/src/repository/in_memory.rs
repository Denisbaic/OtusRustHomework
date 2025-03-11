use std::sync::Arc;

use application::{
    identifier::{NewId, NewIdError},
    repository::{
        device_repository::{self, DeleteError, GetAllError, GetError, Record, SaveError},
        house_repository, light_repository,
        room_repository::{self, Repo},
        thermometer_repository,
    },
};
use async_trait::async_trait;
use domain::entities::{
    device::{Device, LightInfo, ThermometerInfo},
    house::House,
    ids::{DeviceId, HouseId, RoomId},
    room::Room,
};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct InMemoryRepository {
    houses: Arc<Mutex<Vec<House>>>,
    rooms: Arc<Mutex<Vec<Room>>>,
    devices: Arc<Mutex<Vec<Device>>>,
    light_infos: Arc<Mutex<Vec<LightInfo>>>,
    thermometer_infos: Arc<Mutex<Vec<ThermometerInfo>>>,
    last_generated_device_id: Arc<Mutex<DeviceId>>,
    last_generated_room_id: Arc<Mutex<RoomId>>,
    last_generated_house_id: Arc<Mutex<HouseId>>,
}

impl InMemoryRepository {
    pub fn new() -> InMemoryRepository {
        InMemoryRepository {
            houses: Arc::new(Mutex::new(Vec::new())),
            rooms: Arc::new(Mutex::new(Vec::new())),
            devices: Arc::new(Mutex::new(Vec::new())),
            light_infos: Arc::new(Mutex::new(Vec::new())),
            thermometer_infos: Arc::new(Mutex::new(Vec::new())),
            last_generated_device_id: Arc::new(Mutex::new(DeviceId::new(0))),
            last_generated_room_id: Arc::new(Mutex::new(RoomId::new(0))),
            last_generated_house_id: Arc::new(Mutex::new(HouseId::new(0))),
        }
    }
}

#[async_trait]
impl device_repository::Repo for InMemoryRepository {
    async fn get_devices_in_room(&self, room_id: RoomId) -> Result<Vec<Record>, GetAllError> {
        Ok(self
            .devices
            .lock()
            .await
            .iter()
            .filter(|device| device.room_id == room_id)
            .map(|d| Record { device: d.clone() })
            .collect())
    }
    async fn get_device(&self, device_id: DeviceId) -> Result<Record, GetError> {
        self.devices
            .lock()
            .await
            .iter()
            .find(|d| d.id == device_id)
            .cloned()
            .map(|device| Record { device })
            .ok_or(GetError::NotFound)
    }
    async fn save_device(&self, record: Record) -> Result<(), SaveError> {
        if self
            .devices
            .lock()
            .await
            .iter()
            .any(|a| a.id == record.device.id)
            && self.delete_device(record.device.id).await.is_err()
        {
            return Err(application::repository::device_repository::SaveError::Connection);
        }
        self.devices.lock().await.push(record.device.clone());
        Ok(())
    }

    async fn delete_device(&self, device_id: DeviceId) -> Result<(), DeleteError> {
        let mut devices_lock = self.devices.lock().await;
        let device = devices_lock.iter().position(|a| a.id == device_id);
        if let Some(index) = device {
            devices_lock.swap_remove(index);
        }

        self.rooms
            .lock()
            .await
            .iter_mut()
            .for_each(|room| room.devices_ids.retain(|id| *id != device_id));
        Ok(())
    }
}

#[async_trait]
impl room_repository::Repo for InMemoryRepository {
    async fn get_rooms(
        &self,
        house_id: HouseId,
    ) -> Result<
        Vec<application::repository::room_repository::Record>,
        application::repository::room_repository::GetAllError,
    > {
        Ok(self
            .rooms
            .lock()
            .await
            .iter()
            .filter(|room| room.house_id == house_id)
            .map(|r| application::repository::room_repository::Record { room: r.clone() })
            .collect())
    }

    async fn save_room(
        &self,
        record: application::repository::room_repository::Record,
    ) -> Result<(), application::repository::room_repository::SaveError> {
        if self
            .rooms
            .lock()
            .await
            .iter()
            .any(|a| a.id == record.room.id)
            && self.delete_room(record.room.id).await.is_err()
        {
            return Err(application::repository::room_repository::SaveError::Connection);
        }
        self.rooms.lock().await.push(record.room.clone());
        Ok(())
    }

    async fn delete_room(
        &self,
        room_id: RoomId,
    ) -> Result<(), application::repository::room_repository::DeleteError> {
        let mut rooms_lock = self.rooms.lock().await;

        let room = rooms_lock.iter().position(|a| a.id == room_id);
        if let Some(index) = room {
            rooms_lock.swap_remove(index);
        }

        self.devices.lock().await.retain(|device| device.room_id != room_id);
        Ok(())
    }
}

#[async_trait]
impl house_repository::Repo for InMemoryRepository {
    async fn list_houses(
        &self,
    ) -> Result<
        Vec<application::repository::house_repository::Record>,
        application::repository::house_repository::GetAllError,
    > {
        Ok(self
            .houses
            .lock()
            .await
            .iter()
            .map(|h| application::repository::house_repository::Record { house: h.clone() })
            .collect())
    }

    async fn save_house(
        &self,
        record: application::repository::house_repository::Record,
    ) -> Result<(), application::repository::house_repository::SaveError> {
        if self
            .houses
            .lock()
            .await
            .iter()
            .any(|a| a.id == record.house.id)
            && self.delete_house(record.house.id).await.is_err()
        {
            return Err(application::repository::house_repository::SaveError::Connection);
        }
        self.houses.lock().await.push(record.house.clone());
        Ok(())
    }

    async fn delete_house(
        &self,
        house_id: HouseId,
    ) -> Result<(), application::repository::house_repository::DeleteError> {
        let mut house_lock = self.houses.lock().await;
        let house = house_lock.iter().position(|a| a.id == house_id);
        if let Some(index) = house {
            house_lock.swap_remove(index);
        }

        let room_ids_to_delete = self
            .rooms
            .lock()
            .await
            .iter()
            .filter(|room| room.house_id == house_id)
            .map(|room| room.id)
            .collect::<Vec<_>>();

        for room_id in &room_ids_to_delete {
            let _ = self.delete_room(*room_id).await;
        }
        
        self.devices.lock().await.retain(|device| {
            let room_ids_to_delete_ref = &room_ids_to_delete;
            !room_ids_to_delete_ref.iter().any(|room_id| device.room_id == *room_id)
        });

        Ok(())
    }
}

// light_repository не удаляет связанные данные
#[async_trait]
impl light_repository::Repo for InMemoryRepository {
    async fn get_light_info(
        &self,
        device_id: DeviceId,
    ) -> Result<
        application::repository::light_repository::Record,
        application::repository::light_repository::GetError,
    > {
        self.light_infos
            .lock()
            .await
            .iter()
            .find(|a| a.device_id == device_id)
            .cloned()
            .map(|light_info| application::repository::light_repository::Record { light_info })
            .ok_or(application::repository::light_repository::GetError::NotFound)
    }

    async fn save_light_info(
        &self,
        record: application::repository::light_repository::Record,
    ) -> Result<(), application::repository::light_repository::SaveError> {
        let mut light_infos_lock = self.light_infos.lock().await;
        let light_info = light_infos_lock
            .iter()
            .position(|a| a.device_id == record.light_info.device_id);
        if let Some(index) = light_info {
            light_infos_lock.swap_remove(index);
        }
        light_infos_lock.push(record.light_info);
        Ok(())
    }
}

// thermometer_repository не удаляет связанные данные
#[async_trait]
impl thermometer_repository::Repo for InMemoryRepository {
    async fn get_thermometer_info(
        &self,
        device_id: DeviceId,
    ) -> Result<
        application::repository::thermometer_repository::Record,
        application::repository::thermometer_repository::GetError,
    > {
        self.thermometer_infos
            .lock()
            .await
            .iter()
            .find(|a| a.device_id == device_id)
            .cloned()
            .map(
                |thermometer_info| application::repository::thermometer_repository::Record {
                    thermometer_info,
                },
            )
            .ok_or(application::repository::thermometer_repository::GetError::NotFound)
    }
    async fn save_thermometer_info(
        &self,
        record: application::repository::thermometer_repository::Record,
    ) -> Result<(), application::repository::thermometer_repository::SaveError> {
        let mut thermometer_infos_lock = self.thermometer_infos.lock().await;
        let thermometer_info = thermometer_infos_lock
            .iter()
            .position(|a| a.device_id == record.thermometer_info.device_id);
        if let Some(index) = thermometer_info {
            thermometer_infos_lock.swap_remove(index);
        }
        thermometer_infos_lock.push(record.thermometer_info);
        Ok(())
    }
}

#[async_trait]
impl NewId<DeviceId> for InMemoryRepository {
    async fn new_id(&self) -> Result<DeviceId, NewIdError> {
        let mut last_generated_device_id = self.last_generated_device_id.lock().await;

        let device_id_as_u64: u64 = last_generated_device_id.to_owned().into();

        let new_id_result = device_id_as_u64
            .checked_add(1)
            .map(DeviceId::from)
            .ok_or(NewIdError);

        if let Ok(new_id) = new_id_result {
            *last_generated_device_id = new_id;
        }

        new_id_result
    }
}

#[async_trait]
impl NewId<RoomId> for InMemoryRepository {
    async fn new_id(&self) -> Result<RoomId, NewIdError> {
        let mut last_generated_room_id = self.last_generated_room_id.lock().await;

        let room_id_as_u64: u64 = last_generated_room_id.to_owned().into();

        let new_id_result = room_id_as_u64
            .checked_add(1)
            .map(RoomId::from)
            .ok_or(NewIdError);
        
            if let Ok(new_id) = new_id_result {
                *last_generated_room_id = new_id;
            }

        new_id_result
    }
}

#[async_trait]
impl NewId<HouseId> for InMemoryRepository {
    async fn new_id(&self) -> Result<HouseId, NewIdError> {
        let mut last_generated_house_id = self.last_generated_house_id.lock().await;

        let house_id_as_u64: u64 = last_generated_house_id.to_owned().into();

        let new_id_result =   house_id_as_u64
            .checked_add(1)
            .map(HouseId::from)
            .ok_or(NewIdError);

        if let Ok(house_id) = new_id_result {
            *last_generated_house_id = house_id;
        }

        new_id_result
    }
}
