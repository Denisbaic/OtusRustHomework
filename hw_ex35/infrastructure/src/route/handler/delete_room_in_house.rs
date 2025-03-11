use application::repository::{
    device_repository::Repo as DeviceRepository, house_repository::Repo as HouseRepository,
    room_repository::Repo as RoomRepository,
};
use application::usecase::delete_room_in_house;
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
    Path(room_id): Path<u64>,
    State(state): State<AppState<R, I>>,
) -> StatusCode {
    let usecase_dto = delete_room_in_house::Request {
        room_id: room_id.into(),
    };

    let delete_room_in_house = delete_room_in_house::DeleteRoomInHouse::new(&*state.repository);

    let response = delete_room_in_house.exec(usecase_dto).await;

    if let Err(err) = response {
        log::error!("{}", err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}
