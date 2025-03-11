use crate::errors::ProccessorError;

pub(super) trait RequestProcessor {
    fn try_process(
        &self,
        request: &str,
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError>;
}

pub(super) struct HelloProcessor;

impl RequestProcessor for HelloProcessor {
    fn try_process(
        &self,
        request: &str,
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
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
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
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
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
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
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
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
        let device_names: Vec<&str> = room
            .get_devices()
            .iter()
            .map(|value| value.get_device_name())
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
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
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
            .get_device_mut(device_name)
            .ok_or(ProccessorError::CantFindDevice)?;
        match *power_state {
            "true" => device.turn_on(),
            "false" => device.turn_off(),
            _ => device.turn_off(),
        };

        Ok("".to_string())
    }
}

pub(super) struct IsDeviceOnProcessor;

impl RequestProcessor for IsDeviceOnProcessor {
    fn try_process(
        &self,
        request: &str,
        smart_house: &mut smart_house::SmartHouse,
    ) -> Result<String, ProccessorError> {
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
            .get_room_mut(room_name)
            .ok_or(ProccessorError::CantFindRoom)?;
        let device = room
            .get_device_mut(device_name)
            .ok_or(ProccessorError::CantFindDevice)?;

        Ok(format!(
            "room_name:{room_name},device_name:{device_name},is_on:{}",
            device.is_on()
        ))
    }
}
