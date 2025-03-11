use application::{
    repository::{
        device_repository::Repo as DeviceRepository, house_repository::Repo as HouseRepository,
        room_repository::Repo as RoomRepository,
    },
    usecase::read_all_rooms_in_house::{self, RoomDto},
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
    Path(house_id): Path<u64>,
    State(state): State<AppState<R, I>>,
) -> (StatusCode, Json<Response>) {
    let read_all_rooms_in_house =
        read_all_rooms_in_house::ReadAllRoomsInHouse::new(&*state.repository);

    let usecase_dto = read_all_rooms_in_house::Request {
        house_id: house_id.into(),
    };

    let response_dto = read_all_rooms_in_house.exec(usecase_dto).await.unwrap();

    (
        StatusCode::CREATED,
        Json(Response {
            rooms: response_dto.rooms,
        }),
    )
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct Response {
    rooms: Vec<RoomDto>,
}
