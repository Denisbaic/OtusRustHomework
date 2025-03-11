use application::repository::{
    device_repository::Repo as DeviceRepository, house_repository::Repo as HouseRepository,
    room_repository::Repo as RoomRepository,
};
use application::usecase::delete_device_in_room;
use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use domain::entities::ids::{DeviceId, HouseId, RoomId};

use crate::route::AppState;

pub async fn exec<
    R: DeviceRepository + RoomRepository + HouseRepository,
    I: application::identifier::NewId<RoomId>
        + application::identifier::NewId<DeviceId>
        + application::identifier::NewId<HouseId>,
>(
    Path(device_id): Path<u64>,
    State(state): State<AppState<R, I>>,
) -> StatusCode {
    let usecase_dto = delete_device_in_room::Request {
        device_id: device_id.into(),
    };

    let delete_device_in_room = delete_device_in_room::DeleteDeviceInRoom::new(&*state.repository);

    let response = delete_device_in_room.exec(usecase_dto).await;

    if let Err(err) = response {
        log::error!("{}", err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}
