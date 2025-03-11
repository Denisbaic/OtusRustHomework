use std::collections::HashSet;

use application::{
    repository::{
        device_repository::Repo as DeviceRepository, house_repository::Repo as HouseRepository,
        light_repository::Repo as LightRepository, room_repository::Repo as RoomRepository,
        thermometer_repository::Repo as ThermometerRepository,
    },
    usecase::get_report_from_devices,
};
use axum::{extract::State, http::StatusCode, Json};
use domain::entities::ids::{DeviceId, HouseId, RoomId};
use serde::{Deserialize, Serialize};

use crate::route::AppState;

pub async fn exec<
    R: DeviceRepository + RoomRepository + HouseRepository + LightRepository + ThermometerRepository,
    I: application::identifier::NewId<RoomId>
        + application::identifier::NewId<DeviceId>
        + application::identifier::NewId<HouseId>,
>(
    State(state): State<AppState<R, I>>,
    Json(payload): Json<Request>,
) -> (StatusCode, Json<Response>) {
    let get_report_from_devices =
        get_report_from_devices::GetReportFromDevices::new(&*state.repository);

    let usecase_dto = get_report_from_devices::Request {
        device_ids: payload
            .device_ids
            .iter()
            .copied()
            .map(DeviceId::from)
            .collect(),
    };

    let response_dto = get_report_from_devices.exec(usecase_dto).await;

    if let Err(err) = response_dto {
        log::error!("{}", err);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response {
                device_reports: vec![],
            }),
        );
    }

    (
        StatusCode::OK,
        Json(Response {
            device_reports: response_dto.unwrap().device_reports,
        }),
    )
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct Request {
    device_ids: HashSet<u64>,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct Response {
    device_reports: Vec<String>,
}
