#[tokio::main]
async fn main() {
    let server_sddr = "127.0.0.1:8080";
    let mut client = smart_house_client_async::SmartHouseClient::new(server_sddr, "127.0.0.1:8081")
        .await
        .unwrap();
    println!("write \"help\" for print available commands");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();
        if command == "help" {
            println!("Available commands:");
            println!("  help - print available commands");
            println!("  hello");
            println!("  rooms_list");
            println!("  device_report room_name=<string> device_name=<string>");
            println!("  set_device_power_state room_name=<string> device_name=<string> power_state=<true|false>");
            println!("  devices_list room_name=<string>");
            println!("  is_device_on room_name=<string> device_name=<string>");
            println!("  get_device_report_stream room_name=<string> device_name=<string> request_delay=<seconds>");
            println!("  cancel_device_report_stream stream_name=<string>");
            continue;
        }
        if command == "hello" {
            println!("Response from server: {:?}", client.hello_request().await);
            continue;
        }
        if command.starts_with("rooms_list") {
            println!(
                "Response from server: {:?}",
                client.rooms_list_request().await
            );
            continue;
        }
        if command.starts_with("device_report") {
            let params = my_stp_async::custom_parser::parse_request_parameters(command);
            let room_name = params.get("room_name");
            if room_name.is_none() {
                println!("device_report command must have room_name parameter");
                continue;
            }
            let device_name = params.get("device_name");
            if device_name.is_none() {
                println!("device_report command must have device_name parameter");
                continue;
            }
            println!(
                "Response from server: {:?}",
                client
                    .device_report_request(room_name.unwrap(), device_name.unwrap())
                    .await
            );
            continue;
        }
        if command.starts_with("set_device_power_state") {
            let params = my_stp_async::custom_parser::parse_request_parameters(command);
            let room_name = params.get("room_name");
            if room_name.is_none() {
                println!("set_device_power_state command must have room_name parameter");
                continue;
            }
            let device_name = params.get("device_name");
            if device_name.is_none() {
                println!("set_device_power_state command must have device_name parameter");
                continue;
            }
            let power_state = params.get("power_state");
            if power_state.is_none() {
                println!("set_device_power_state command must have power_state parameter");
                continue;
            }
            println!(
                "Response from server: {:?}",
                client
                    .set_device_power_state_request(
                        room_name.unwrap(),
                        device_name.unwrap(),
                        power_state.unwrap().parse().unwrap()
                    )
                    .await
            );
            continue;
        }
        if command.starts_with("devices_list") {
            let params = my_stp_async::custom_parser::parse_request_parameters(command);
            let room_name = params.get("room_name");
            if room_name.is_none() {
                println!("devices_list command must have room_name parameter");
                continue;
            }
            println!(
                "Response from server: {:?}",
                client.devices_list_request(room_name.unwrap()).await
            );
            continue;
        }
        if command.starts_with("is_device_on") {
            let params = my_stp_async::custom_parser::parse_request_parameters(command);
            let room_name = params.get("room_name");
            if room_name.is_none() {
                println!("is_device_on command must have room_name parameter");
                continue;
            }
            let device_name = params.get("device_name");
            if device_name.is_none() {
                println!("is_device_on command must have device_name parameter");
                continue;
            }
            println!(
                "Response from server: {:?}",
                client
                    .is_device_on_request(room_name.unwrap(), device_name.unwrap())
                    .await
            );
            continue;
        }
        if command.starts_with("get_device_report_stream") {
            let params = my_stp_async::custom_parser::parse_request_parameters(command);
            let room_name = params.get("room_name");
            if room_name.is_none() {
                println!("get_device_report_stream command must have room_name parameter");
                continue;
            }
            let device_name = params.get("device_name");
            if device_name.is_none() {
                println!("get_device_report_stream command must have device_name parameter");
                continue;
            }
            let request_delay = params.get("request_delay");
            let request_delay: Option<u64> =
                request_delay.map(|request_delay| request_delay.parse().unwrap());

            println!(
                "Response from server: {:?}",
                client
                    .get_device_report_stream_request(
                        room_name.unwrap(),
                        device_name.unwrap(),
                        request_delay
                    )
                    .await
            );
            continue;
        }
        if command.starts_with("cancel_device_report_stream") {
            let params = my_stp_async::custom_parser::parse_request_parameters(command);
            let stream_name = params.get("stream_name");
            if stream_name.is_none() {
                println!("cancel_device_report_stream command must have stream_name parameter");
                continue;
            }
            println!(
                "Response from server: {:?}",
                client
                    .cancel_device_report_stream_request(stream_name.unwrap())
                    .await
            );
            continue;
        }
        println!("no command found");
    }
}
