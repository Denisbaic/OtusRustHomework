use application::{
    repository::{
        device_repository::Repo as DeviceRepository, house_repository::Repo as HouseRepository,
        room_repository::Repo as RoomRepository,
    },
    usecase::read_all_houses::{self, HouseDto},
};
use axum::{extract::State, http::StatusCode, Json};
use domain::entities::ids::{DeviceId, HouseId, RoomId};
use serde::Serialize;

use crate::route::AppState;

pub async fn exec<
    R: DeviceRepository + RoomRepository + HouseRepository,
    I: application::identifier::NewId<RoomId>
        + application::identifier::NewId<DeviceId>
        + application::identifier::NewId<HouseId>,
>(
    State(state): State<AppState<R, I>>,
) -> (StatusCode, Json<Response>) {
    let read_all_houses_usecase = read_all_houses::ReadAllHouses::new(&*state.repository);

    let response_dto = read_all_houses_usecase.exec().await.unwrap();

    (
        StatusCode::CREATED,
        Json(Response {
            houses: response_dto.houses,
        }),
    )
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct Response {
    houses: Vec<HouseDto>,
}
