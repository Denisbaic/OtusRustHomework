mod handler;

use std::sync::Arc;

use application::repository::house_repository::Repo as HouseRepository;
use application::repository::room_repository::Repo as RoomRepository;
use application::repository::thermometer_repository::Repo as ThermometerRepository;
use application::repository::{
    device_repository::Repo as DeviceRepository, light_repository::Repo as LightRepository,
};
use axum::routing::delete;
use axum::{
    routing::{get, post},
    Router,
};
use domain::entities::ids::{DeviceId, HouseId, RoomId};

#[derive(Clone)]
pub struct AppState<
    R: DeviceRepository + RoomRepository + HouseRepository + 'static,
    I: application::identifier::NewId<RoomId>
        + application::identifier::NewId<DeviceId>
        + application::identifier::NewId<HouseId>
        + 'static,
> {
    pub repository: Arc<R>,
    pub id_generator: Arc<I>,
}

pub fn get_route<
    R: DeviceRepository
        + RoomRepository
        + HouseRepository
        + LightRepository
        + ThermometerRepository
        + Clone,
    I: application::identifier::NewId<RoomId>
        + application::identifier::NewId<DeviceId>
        + application::identifier::NewId<HouseId>
        + Clone,
>(
    app_state: AppState<R, I>,
) -> Router {
    Router::new()
        // Получение всех домов
        .route("/houses", get(handler::read_all_houses::exec))
        // Получение и создание комнат в конкретном доме
        .route(
            "/houses/{house_id}/rooms",
            get(handler::read_all_rooms_in_house::exec).post(handler::create_room_in_house::exec),
        )
        // Удаление комнаты
        .route(
            "/delete_room/{room_id}",
            delete(handler::delete_room_in_house::exec),
        )
        // Получение и создание устройств в конкретной комнате
        .route(
            "/rooms/{room_id}/devices",
            get(handler::read_all_devices_in_room::exec).post(handler::create_device_in_room::exec),
        )
        // Удаление устройства
        .route(
            "/devices/{device_id}",
            delete(handler::delete_device_in_room::exec),
        )
        // Получение отчёта с устройств
        .route(
            "/devices/report",
            post(handler::get_report_from_devices::exec),
        )
        .route("/", get(hello_world))
        .with_state(app_state)
}

// Добавление состояния приложения является опциональным
async fn hello_world() -> &'static str {
    "Hello world!"
}
