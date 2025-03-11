use application::{
    repository::{
        device_repository::Repo as DeviceRepository, house_repository::Repo as HouseRepository,
        room_repository::Repo as RoomRepository,
    },
    usecase::read_all_devices_in_room,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use domain::entities::ids::{DeviceId, HouseId, RoomId};
use serde::Serialize;

use crate::route::AppState;

pub async fn exec<
    R: DeviceRepository + RoomRepository + HouseRepository,
    I: application::identifier::NewId<RoomId>
        + application::identifier::NewId<DeviceId>
        + application::identifier::NewId<HouseId>,
>(
    Path(room_id): Path<u64>,
    State(state): State<AppState<R, I>>,
) -> (StatusCode, Json<Response>) {
    let read_all_devices_in_room =
    read_all_devices_in_room::ReadAllDevicesInRoom::new(&*state.repository);

    let usecase_dto = read_all_devices_in_room::Request {
        room_id: room_id.into()
    };

    let response_dto = read_all_devices_in_room.exec(usecase_dto).await.unwrap();

    (
        StatusCode::CREATED,
        Json(Response {
            devices: response_dto.devices.iter().map(|d| d.id).collect(),
        }),
    )
}
// the output to our `create_user` handler
#[derive(Serialize)]
pub struct Response {
    devices: Vec<u64>,
}
