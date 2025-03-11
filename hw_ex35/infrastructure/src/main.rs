use std::sync::Arc;

use domain::entities::house::House;
use application::{identifier::NewId, repository::house_repository::Repo};

mod repository;
mod route;

#[tokio::main]
async fn main() {
    let repo = Arc::new(repository::in_memory::InMemoryRepository::new());

    repo.save_house(
        application::repository::house_repository::Record { house:
        House{ id: repo.new_id().await.unwrap(), name: String::from("test")} }).await.unwrap();

    let app_state = route::AppState {
        repository: repo.clone(),
        id_generator: repo,
    };

    let app = route::get_route(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
    println!("Shutting down");
}
