fn main() {
    let client = smart_house_client::SmartHouseClient::new("127.0.0.1:8080");

    println!("Response from server: {}", client.hello_request().unwrap());
}
