#[tokio::main]
async fn main() {
    let mut server =
        smart_house_server_async::SmartHouseServer::new("127.0.0.1:8080", "127.0.0.1:8082")
            .await
            .unwrap();
    server.start_server_listening();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("server main thread tick")
    }
}
