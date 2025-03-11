use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use crate::{errors::ProccessorError, ServerStore};

pub(super) trait RequestProcessor: Sync + Send {
    fn try_process(
        &self,
        request: &str,
        server: Arc<RwLock<ServerStore>>,
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError>;
}

pub(super) struct HelloProcessor;

impl RequestProcessor for HelloProcessor {
    fn try_process(
        &self,
        request: &str,
        server: Arc<RwLock<ServerStore>>,
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
        let _ = server;
        let _ = smart_house;

        if !request.starts_with("hello") {
            return Err(ProccessorError::CantProccessRequest);
        }
        Ok("Hello from server".to_string())
    }
}

pub(super) struct DeviceReportProcessor;

impl RequestProcessor for DeviceReportProcessor {
    fn try_process(
        &self,
        request: &str,
        server: Arc<RwLock<ServerStore>>,
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
        let _ = server;
        let _ = smart_house;

        if !request.starts_with("device_report") {
            return Err(ProccessorError::CantProccessRequest);
        }

        let params = my_stp::custom_parser::parse_request_parameters(request);
        let room_name = params
            .get("room_name")
            .ok_or(ProccessorError::CantProccessRequest)?;
        let device_name = params
            .get("device_name")
            .ok_or(ProccessorError::CantProccessRequest)?;

        let response = smart_house
            .create_report_by_devices(vec![(room_name, device_name)])
            .map_err(|_| ProccessorError::CantGetReport)?;

        Ok(response)
    }
}

pub(super) struct RoomsListProcessor;

impl RequestProcessor for RoomsListProcessor {
    fn try_process(
        &self,
        request: &str,
        server: Arc<RwLock<ServerStore>>,
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
        let _ = server;
        if !request.starts_with("rooms_list") {
            return Err(ProccessorError::CantProccessRequest);
        }

        let rooms = smart_house.get_rooms();
        let room_names: Vec<&str> = rooms.iter().map(|value| value.name()).collect();
        let room_names_string = room_names.join(",");

        let response = format!("[{room_names_string}]");

        Ok(response)
    }
}

pub(super) struct DeviceListProcessor;

impl RequestProcessor for DeviceListProcessor {
    fn try_process(
        &self,
        request: &str,
        server: Arc<RwLock<ServerStore>>,
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
        let _ = server;
        if !request.starts_with("devices_list") {
            return Err(ProccessorError::CantProccessRequest);
        }

        let params = my_stp::custom_parser::parse_request_parameters(request);
        let room_name = params
            .get("room_name")
            .ok_or(ProccessorError::CantProccessRequest)?;

        let room = smart_house
            .get_room(room_name)
            .ok_or(ProccessorError::CantFindRoom)?;

        let device_names: Vec<String> = room
            .get_devices()
            .iter()
            .map(|device| device.read().unwrap().get_device_name().to_string())
            .collect();
        let device_names_string = device_names.join(",");

        let response = format!("{room_name}:[{device_names_string}]");

        Ok(response)
    }
}

pub(super) struct SetDevicePowerStateProcessor;

impl RequestProcessor for SetDevicePowerStateProcessor {
    fn try_process(
        &self,
        request: &str,
        server: Arc<RwLock<ServerStore>>,
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
        let _ = server;
        if !request.starts_with("set_device_power_state") {
            return Err(ProccessorError::CantProccessRequest);
        }

        let params = my_stp::custom_parser::parse_request_parameters(request);
        let room_name = params
            .get("room_name")
            .ok_or(ProccessorError::CantProccessRequest)?;
        let device_name = params
            .get("device_name")
            .ok_or(ProccessorError::CantProccessRequest)?;
        let power_state = params
            .get("power_state")
            .ok_or(ProccessorError::CantProccessRequest)?;

        let room = smart_house
            .get_room_mut(room_name)
            .ok_or(ProccessorError::CantFindRoom)?;
        let device = room
            .get_device(device_name)
            .ok_or(ProccessorError::CantFindDevice)?;

        let mut device_write = device.write().unwrap();
        match *power_state {
            "true" => device_write.turn_on(),
            "false" => device_write.turn_off(),
            _ => device_write.turn_off(),
        };

        Ok("".to_string())
    }
}

pub(super) struct IsDeviceOnProcessor;

impl RequestProcessor for IsDeviceOnProcessor {
    fn try_process(
        &self,
        request: &str,
        server: Arc<RwLock<ServerStore>>,
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
        let _ = server;
        if !request.starts_with("is_device_on") {
            return Err(ProccessorError::CantProccessRequest);
        }

        let params = my_stp::custom_parser::parse_request_parameters(request);
        let room_name = params
            .get("room_name")
            .ok_or(ProccessorError::CantProccessRequest)?;
        let device_name = params
            .get("device_name")
            .ok_or(ProccessorError::CantProccessRequest)?;

        let room = smart_house
            .get_room(room_name)
            .ok_or(ProccessorError::CantFindRoom)?;

        let device = room
            .get_device(device_name)
            .ok_or(ProccessorError::CantProccessRequest)?;

        Ok(format!(
            "room_name:{room_name},device_name:{device_name},is_on:{}",
            device.read().unwrap().is_on()
        ))
    }
}

pub(super) struct GetDeviceReportStreamProcessor;

impl RequestProcessor for GetDeviceReportStreamProcessor {
    fn try_process(
        &self,
        request: &str,
        server: Arc<RwLock<ServerStore>>,
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
        if !request.starts_with("get_device_report_stream") {
            return Err(ProccessorError::CantProccessRequest);
        }
        println!("get get_device_report_stream request");
        const DEFAULT_REQUEST_DELAY: u64 = 5;

        let params = my_stp::custom_parser::parse_request_parameters(request);
        let room_name = params
            .get("room_name")
            .ok_or(ProccessorError::CantProccessRequest)?;
        let device_name = params
            .get("device_name")
            .ok_or(ProccessorError::CantProccessRequest)?;
        let request_delay: u64 = params.get("request_delay").map_or_else(
            || DEFAULT_REQUEST_DELAY,
            |request_delay_string| {
                request_delay_string
                    .parse()
                    .unwrap_or(DEFAULT_REQUEST_DELAY)
            },
        );
        let addr_for_send = params
            .get("addr")
            .ok_or(ProccessorError::CantProccessRequest)?
            .to_string();

        let room = smart_house
            .get_room_mut(room_name)
            .ok_or(ProccessorError::CantFindRoom)?;
        let device = room
            .get_device(device_name)
            .ok_or(ProccessorError::CantFindDevice)?;
        let server_thread = server.clone();

        let thread_name = format!("{}-{}", room_name, device.read().unwrap().get_device_name());
        let (canceler, cancellation_token) = thread_cancellation_token::cancellation_token();
        let _ = thread::spawn(move || {
            let address_for_reporting = addr_for_send;
            loop {
                println!("thread loop");
                if cancellation_token.should_cancel() {
                    println!("should thread canceled");
                    break;
                }
                thread::sleep(Duration::from_secs(request_delay));
                if cancellation_token.should_cancel() {
                    println!("should thread canceled");
                    break;
                }

                if let Ok(report) = { device.read().unwrap().create_report() } {
                    println!(
                        "send report to {} : {:?}",
                        address_for_reporting.clone(),
                        server_thread
                            .read()
                            .unwrap()
                            .udp_socket
                            .send_to(report.as_bytes(), address_for_reporting.clone())
                    );
                }
            }
        });

        let mut server = server.write().unwrap();
        let entry = server.execution_threads.remove(&thread_name);
        if let Some(cancellable_thread) = entry {
            cancellable_thread.cancel();
        }

        server
            .execution_threads
            .insert(thread_name.clone(), canceler);

        Ok(format!("create thread with name : {thread_name}"))
    }
}

pub(super) struct CancelDeviceReportStreamProcessor;

impl RequestProcessor for CancelDeviceReportStreamProcessor {
    fn try_process(
        &self,
        request: &str,
        server: Arc<RwLock<ServerStore>>,
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
        let _ = smart_house;
        if !request.starts_with("cancel_device_report_stream") {
            return Err(ProccessorError::CantProccessRequest);
        }

        let params = my_stp::custom_parser::parse_request_parameters(request);
        let thread_name = params
            .get("stream_name")
            .ok_or(ProccessorError::CantProccessRequest)?;

        if let Some(ca) = server
            .write()
            .unwrap()
            .execution_threads
            .remove(*thread_name)
        {
            println!("Start joining thread with name : {thread_name}");
            ca.cancel();
            return Ok(format!("Cancel thread with name : {thread_name}"));
        }
        Ok(format!(
            "Cancel thread with name : {thread_name} - no thread to cancel"
        ))
    }
}
