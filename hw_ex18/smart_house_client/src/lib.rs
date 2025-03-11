use std::net::ToSocketAddrs;

use my_stp::errors::RequestError;

pub struct SmartHouseClient<Addrs>
where
    Addrs: ToSocketAddrs + Clone,
{
    addrs: Addrs,
}

impl<Addrs> SmartHouseClient<Addrs>
where
    Addrs: ToSocketAddrs + Clone,
{
    pub fn new(addrs: Addrs) -> Self {
        Self { addrs }
    }

    pub fn hello_request(&self) -> Result<String, RequestError> {
        let mut connect = my_stp::client::StpClient::connect(self.addrs.clone())?;

        let request_string = "hello";
        connect.send_request(request_string)
    }

    pub fn device_report_request(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<String, RequestError> {
        let mut connect = my_stp::client::StpClient::connect(self.addrs.clone())?;

        let request_string =
            format!("device_report room_name:{room_name} device_name:{device_name}");
        connect.send_request(request_string)
    }

    pub fn rooms_list_request(&self) -> Result<String, RequestError> {
        let mut connect = my_stp::client::StpClient::connect(self.addrs.clone())?;

        let request_string = "rooms_list";
        connect.send_request(request_string)
    }

    pub fn devices_list_request(&self, room_name: &str) -> Result<String, RequestError> {
        let mut connect = my_stp::client::StpClient::connect(self.addrs.clone())?;

        let request_string = format!("devices_list room_name:{room_name}");
        connect.send_request(request_string)
    }

    pub fn is_device_on_request(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<String, RequestError> {
        let mut connect = my_stp::client::StpClient::connect(self.addrs.clone())?;

        let request_string =
            format!("is_device_on room_name:{room_name} device_name:{device_name}");
        connect.send_request(request_string)
    }

    pub fn set_device_power_state_request(
        &self,
        room_name: &str,
        device_name: &str,
        power_state: bool,
    ) -> Result<String, RequestError> {
        let mut connect = my_stp::client::StpClient::connect(self.addrs.clone())?;

        let request_string = format!("set_device_power_state room_name:{room_name} device_name:{device_name} power_state:{power_state}");
        connect.send_request(request_string)
    }
}
