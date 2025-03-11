fn main() {
    let client = smart_house_client::SmartHouseClient::new("127.0.0.1:8080");
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
            println!("  device_report room_name:<string> device_name:<string>");
            println!("  set_device_power_state room_name:<string> device_name:<string> power_state:<true|false>");
            println!("  devices_list room_name:<string>");
            println!("  is_device_on room_name:<string> device_name:<string>");
            continue;
        }
        if command == "hello" {
            println!("Response from server: {:?}", client.hello_request());
            continue;
        }
        if command.starts_with("rooms_list") {
            println!("Response from server: {:?}", client.rooms_list_request());
            continue;
        }
        if command.starts_with("device_report") {
            let params = my_stp::custom_parser::parse_request_parameters(command);
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
                client.device_report_request(room_name.unwrap(), device_name.unwrap())
            );
            continue;
        }
        if command.starts_with("set_device_power_state") {
            let params = my_stp::custom_parser::parse_request_parameters(command);
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
                client.set_device_power_state_request(
                    room_name.unwrap(),
                    device_name.unwrap(),
                    power_state.unwrap().parse().unwrap()
                )
            );
            continue;
        }
        if command.starts_with("devices_list") {
            let params = my_stp::custom_parser::parse_request_parameters(command);
            let room_name = params.get("room_name");
            if room_name.is_none() {
                println!("devices_list command must have room_name parameter");
                continue;
            }
            println!(
                "Response from server: {:?}",
                client.devices_list_request(room_name.unwrap())
            );
            continue;
        }
        if command.starts_with("is_device_on") {
            let params = my_stp::custom_parser::parse_request_parameters(command);
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
                client.is_device_on_request(room_name.unwrap(), device_name.unwrap())
            );
            continue;
        }
        println!("no command found");
    }
}
