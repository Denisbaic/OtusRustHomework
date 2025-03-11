use crate::{device::Device, reporter::Reporter};
use core::f32;
use std::{rc::Rc, str::FromStr};

pub trait SmartSocketInfoProvider {
    fn get_current_power_consumption(&self) -> f32;
}

pub struct SmartSocket {
    name: String,
    smart_socket_info_provider: Rc<dyn SmartSocketInfoProvider>,
    is_on: bool,
}

impl SmartSocket {
    pub fn new(
        name: &str,
        smart_socket_info_provider: Rc<dyn SmartSocketInfoProvider>,
    ) -> SmartSocket {
        SmartSocket {
            name: String::from_str(name).expect("Я не знаю что тут пошло не так"),
            smart_socket_info_provider,
            is_on: true,
        }
    }

    pub fn get_current_power_consumption(&self) -> Option<f32> {
        match self.is_on {
            true => Some(
                self.smart_socket_info_provider
                    .get_current_power_consumption(),
            ),
            false => None,
        }
    }
}

impl Device for SmartSocket {
    fn turn_on(&mut self) {
        self.is_on = true;
    }

    fn turn_off(&mut self) {
        self.is_on = false;
    }

    fn is_on(&self) -> bool {
        self.is_on
    }

    fn is_off(&self) -> bool {
        !self.is_on
    }

    fn get_device_name(&self) -> &str {
        &self.name
    }
}

impl Reporter for SmartSocket {
    fn create_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        if !self.is_on() {
            return Err("SmartSocket is off".into());
        }

        match self.get_current_power_consumption() {
            None => return Err("SmartSockerReporterError::PowerCannotBeParsed".into()),
            Some(power) if power.is_finite() => {}
            _ => return Err("SmartSockerReporterError::PowerCannotBeParsed".into()),
        }

        let report_title = format!("---------{}---------", self.name);
        let result_power = self.get_current_power_consumption().unwrap_or(f32::NAN);
        Ok(format!(
            "{report_title}\n Текущая потребляемая мощность: {result_power} Вт\n{}",
            "-".repeat(report_title.chars().count())
        ))
    }
}

#[cfg(test)]
mod smart_socket_tests {
    use super::*;

    struct MockSmartSocketInfoProvider;

    impl SmartSocketInfoProvider for MockSmartSocketInfoProvider {
        fn get_current_power_consumption(&self) -> f32 {
            10.0
        }
    }

    #[test]
    fn test_smart_socket_creation() {
        let info_provider = Rc::new(MockSmartSocketInfoProvider);
        let socket = SmartSocket::new("Test Socket", info_provider);
        assert_eq!(socket.name, "Test Socket");
        assert!(socket.is_on);
    }

    #[test]
    fn test_get_current_power_consumption() {
        let info_provider = Rc::new(MockSmartSocketInfoProvider);
        let socket = SmartSocket::new("Test Socket", info_provider);
        assert_eq!(socket.get_current_power_consumption(), Some(10.0));
    }

    #[test]
    fn test_turn_off_socket() {
        let info_provider = Rc::new(MockSmartSocketInfoProvider);
        let mut socket = SmartSocket::new("Test Socket", info_provider);
        socket.turn_off();
        assert!(socket.is_off());
        socket.turn_off();
        assert!(socket.is_off());
    }

    #[test]
    fn test_change_toggle_socket() {
        let info_provider = Rc::new(MockSmartSocketInfoProvider);
        let mut socket = SmartSocket::new("Test Socket", info_provider);
        assert!(socket.is_on());
        socket.toggle();
        assert!(socket.is_off());
        socket.toggle();
        assert!(socket.is_on());
    }

    #[test]
    fn test_change_turn_on() {
        let info_provider = Rc::new(MockSmartSocketInfoProvider);
        let mut socket = SmartSocket::new("Test Socket", info_provider);
        assert!(socket.is_on());
        socket.turn_off();
        assert!(socket.is_off());
        socket.turn_off();
        assert!(socket.is_off());
    }
}
