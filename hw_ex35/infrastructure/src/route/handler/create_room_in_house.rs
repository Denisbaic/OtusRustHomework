use application::repository::{
    device_repository::Repo as DeviceRepository, house_repository::Repo as HouseRepository,
    room_repository::Repo as RoomRepository,
};
use application::usecase::create_room_in_house::{self, CreateRoomInHouse};
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
use domain::entities::ids::{DeviceId, HouseId, RoomId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::route::AppState;

pub async fn exec<
    R: DeviceRepository + RoomRepository + HouseRepository,
    I: application::identifier::NewId<RoomId>
        + application::identifier::NewId<DeviceId>
        + application::identifier::NewId<HouseId>,
>(
    Path(house_id): Path<u64>,
    State(state): State<AppState<R, I>>,
    Json(payload): Json<Request>,
) -> (StatusCode, Json<Response>) {
    let usecase_dto = create_room_in_house::Request {
        house_id: house_id.into(),
        name: payload.name,
        devices_ids: HashSet::new(),
    };

    let create_room_in_house = CreateRoomInHouse::new(&*state.repository, &*state.id_generator);

    create_room_in_house.exec(usecase_dto).await.unwrap();

    (StatusCode::CREATED, Json(Response))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct Request {
    pub name: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct Response;
