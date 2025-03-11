fn main() {
    let mut server = smart_house_server::SmartHouseServer::new("127.0.0.1:8080").unwrap();
    server.start_server_listening();
}
