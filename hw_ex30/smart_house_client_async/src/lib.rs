use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    sync::watch::{self, Sender},
};

use my_stp_async::errors::RequestError;

pub struct SmartHouseClient<Addrs>
where
    Addrs: ToSocketAddrs + Clone + ToString,
{
    server_addr: Addrs,
    udp_socket_addr: Addrs,
    udp_thread: Sender<bool>,
}

impl<Addrs> SmartHouseClient<Addrs>
where
    Addrs: ToSocketAddrs + Clone + ToString,
{
    pub async fn new(
        server_addr: Addrs,
        udp_socket_addr: Addrs,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let udp_socket = UdpSocket::bind(udp_socket_addr.clone()).await?;

        let (canceller, cancellation_token) = watch::channel(false);
        tokio::spawn(async move {
            let mut buf = [0u8; 1024 * 4];
            loop {
                if *cancellation_token.borrow() {
                    break;
                }

                match udp_socket.recv_from(&mut buf).await {
                    Ok((size, addr)) => {
                        let message = String::from_utf8_lossy(&buf[..size]);
                        println!("Received from {}: {}", addr, message);
                    }
                    Err(e) => {
                        eprintln!("Error receiving data: {}", e);
                    }
                }
            }
        });

        Ok(Self {
            server_addr,
            udp_socket_addr,
            udp_thread: canceller,
        })
    }

    pub async fn hello_request(&self) -> Result<String, RequestError> {
        let mut connect =
            my_stp_async::client::StpClient::connect(self.server_addr.clone()).await?;

        let request_string = "hello";
        connect.send_request(request_string).await
    }

    pub async fn device_report_request(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<String, RequestError> {
        let mut connect =
            my_stp_async::client::StpClient::connect(self.server_addr.clone()).await?;

        let request_string =
            format!("device_report room_name={room_name} device_name={device_name}");
        connect.send_request(request_string).await
    }

    pub async fn rooms_list_request(&self) -> Result<String, RequestError> {
        let mut connect =
            my_stp_async::client::StpClient::connect(self.server_addr.clone()).await?;

        let request_string = "rooms_list";
        connect.send_request(request_string).await
    }

    pub async fn devices_list_request(&self, room_name: &str) -> Result<String, RequestError> {
        let mut connect =
            my_stp_async::client::StpClient::connect(self.server_addr.clone()).await?;

        let request_string = format!("devices_list room_name={room_name}");
        connect.send_request(request_string).await
    }

    pub async fn is_device_on_request(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<String, RequestError> {
        let mut connect =
            my_stp_async::client::StpClient::connect(self.server_addr.clone()).await?;

        let request_string =
            format!("is_device_on room_name={room_name} device_name={device_name}");
        connect.send_request(request_string).await
    }

    pub async fn set_device_power_state_request(
        &self,
        room_name: &str,
        device_name: &str,
        power_state: bool,
    ) -> Result<String, RequestError> {
        let mut connect =
            my_stp_async::client::StpClient::connect(self.server_addr.clone()).await?;

        let request_string = format!("set_device_power_state room_name={room_name} device_nam={device_name} power_state={power_state}");
        connect.send_request(request_string).await
    }

    pub async fn get_device_report_stream_request(
        &mut self,
        room_name: &str,
        device_name: &str,
        request_delay_seconds: Option<u64>,
    ) -> Result<String, RequestError> {
        let result_request_delay = request_delay_seconds.unwrap_or(5);
        let addr_as_string = self.udp_socket_addr.to_string();
        let mut connect =
            my_stp_async::client::StpClient::connect(self.server_addr.clone()).await?;

        let request_string = format!("get_device_report_stream room_name={room_name} device_name={device_name} request_delay={result_request_delay} addr={addr_as_string}");

        connect.send_request(request_string).await
    }

    pub async fn cancel_device_report_stream_request(
        &mut self,
        stream_name: &str,
    ) -> Result<String, RequestError> {
        let mut connect =
            my_stp_async::client::StpClient::connect(self.server_addr.clone()).await?;

        let request_string = format!("cancel_device_report_stream stream_name={stream_name}");
        connect.send_request(request_string).await
    }
}

impl<Addrs> Drop for SmartHouseClient<Addrs>
where
    Addrs: ToSocketAddrs + Clone + ToString,
{
    fn drop(&mut self) {
        match self.udp_thread.send(true) {
            Ok(_) => {}
            Err(e) => println!("Error sending cancellation signal: {}", e),
        }
    }
}
