use std::rc::Rc;
use std::str::FromStr;

use crate::device::Device;
use crate::reporter::Reporter;
use crate::temperature::{Temperature, TemperatureMeasureUnits};

pub trait ThermometerInfoProvider {
    fn get_temperature(&self) -> Temperature;
}

pub struct Thermometer {
    name: String,
    thermometer_info_provider: Rc<dyn ThermometerInfoProvider>,
    is_on: bool,
}

impl Thermometer {
    pub fn new(
        name: &str,
        thermometer_info_provider: Rc<dyn ThermometerInfoProvider>,
    ) -> Thermometer {
        Thermometer {
            name: String::from_str(name).expect("Я не знаю что тут пошло не так"),
            thermometer_info_provider,
            is_on: true,
        }
    }

    pub fn get_temperature(
        &self,
        _temperature_units: TemperatureMeasureUnits,
    ) -> Option<Temperature> {
        match self.is_on {
            true => Some(self.thermometer_info_provider.get_temperature()),
            false => None,
        }
    }
}

impl Device for Thermometer {
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

impl Reporter for Thermometer {
    fn create_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        if !self.is_on() {
            return Err("ThermometerIsOff".into());
        }

        match self.get_temperature(TemperatureMeasureUnits::Celsius) {
            None => return Err("TemperatureCannotBeParsed".into()),
            Some(temp) if temp.get_value().is_finite() => {}
            _ => return Err("TemperatureCannotBeParsed".into()),
        }

        let report_title = format!("---------{}---------", self.name);
        let result_temperature = self
            .get_temperature(TemperatureMeasureUnits::Celsius)
            .unwrap_or(Temperature::new(f32::NAN, TemperatureMeasureUnits::Celsius));
        Ok(format!(
            "{report_title}\n Температура: {result_temperature}\n{}",
            "-".repeat(report_title.chars().count())
        ))
    }
}

#[cfg(test)]
mod smart_socket_tests {
    use super::*;
    use crate::temperature::{Temperature, TemperatureMeasureUnits};

    struct MockThermometerInfoProvider {
        temperature: Temperature,
    }

    impl ThermometerInfoProvider for MockThermometerInfoProvider {
        fn get_temperature(&self) -> Temperature {
            self.temperature
        }
    }

    #[test]
    fn test_thermometer_creation() {
        let thermometer_info_provider = Rc::new(MockThermometerInfoProvider {
            temperature: Temperature::new(10.0, TemperatureMeasureUnits::Celsius),
        });
        let thermometer = Thermometer::new("Test Thermometer", thermometer_info_provider);
        assert_eq!(thermometer.name, "Test Thermometer");
        assert!(thermometer.is_on);
    }

    #[test]
    fn test_thermometer_get_temperature() {
        let thermometer_info_provider = Rc::new(MockThermometerInfoProvider {
            temperature: Temperature::new(10.0, TemperatureMeasureUnits::Celsius),
        });

        let thermometer = Thermometer::new("Test Thermometer", thermometer_info_provider);
        let temperature = thermometer.get_temperature(TemperatureMeasureUnits::Celsius);
        assert!(temperature.is_some());

        let temperature = temperature.unwrap();
        assert_eq!(temperature.get_value(), 10.0);
        assert_eq!(
            temperature.get_measure_units(),
            TemperatureMeasureUnits::Celsius
        );
    }

    #[test]
    fn test_thermometer_turn_on_and_off() {
        let thermometer_info_provider = Rc::new(MockThermometerInfoProvider {
            temperature: Temperature::new(10.0, TemperatureMeasureUnits::Celsius),
        });
        let mut thermometer = Thermometer::new("Test Thermometer", thermometer_info_provider);
        thermometer.turn_off();
        assert!(!thermometer.is_on);
        thermometer.turn_on();
        assert!(thermometer.is_on);
    }

    #[test]
    fn test_thermometer_get_temperature_when_off() {
        let thermometer_info_provider = Rc::new(MockThermometerInfoProvider {
            temperature: Temperature::new(10.0, TemperatureMeasureUnits::Celsius),
        });
        let mut thermometer = Thermometer::new("Test Thermometer", thermometer_info_provider);
        thermometer.turn_off();
        let temperature = thermometer.get_temperature(TemperatureMeasureUnits::Celsius);
        assert!(temperature.is_none());
    }
}
