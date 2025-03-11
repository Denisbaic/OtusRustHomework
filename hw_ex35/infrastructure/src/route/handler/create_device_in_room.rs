use application::repository::{
    device_repository::Repo as DeviceRepository, house_repository::Repo as HouseRepository,
    light_repository::Repo as LightRepository, room_repository::Repo as RoomRepository,
    thermometer_repository::Repo as ThermometerRepository,
};
use application::usecase::create_device_in_room;
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
use domain::entities::ids::{DeviceId, HouseId, RoomId};
use serde::Deserialize;

use crate::route::AppState;

pub async fn exec<
    R: DeviceRepository + RoomRepository + HouseRepository + LightRepository + ThermometerRepository,
    I: application::identifier::NewId<RoomId>
        + application::identifier::NewId<DeviceId>
        + application::identifier::NewId<HouseId>,
>(
    Path(room_id): Path<u64>,
    State(state): State<AppState<R, I>>,
    Json(payload): Json<Request>,
) -> StatusCode {
    let usecase_dto = create_device_in_room::Request {
        room_id: room_id.into(),
        device_name: payload.device_name,
        device_type: payload.device_type,
    };

    let create_device_in_room =
        create_device_in_room::CreateDeviceInRoom::new(&*state.repository, &*state.id_generator);

    let response = create_device_in_room.exec(usecase_dto).await;

    if let Err(err) = response {
        log::error!("{}", err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

#[derive(Deserialize)]
pub struct Request {
    pub device_name: String,
    pub device_type: String,
}
