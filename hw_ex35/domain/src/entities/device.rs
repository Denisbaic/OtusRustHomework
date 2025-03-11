use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::entities::ids::{DeviceId, RoomId};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeviceType {
    Thermometer,
    Light,
}

#[derive(Debug, Clone)]
pub struct Device {
    pub room_id: RoomId,
    pub id: DeviceId,
    pub name: String,
    pub device_type: DeviceType,
}

#[derive(Debug, Clone, Copy)]
pub struct ThermometerInfo {
    pub device_id: DeviceId,
    pub temperature: Temperature,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureMeasureUnits {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Display for TemperatureMeasureUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        match self {
            TemperatureMeasureUnits::Celsius => write!(f, "°C"),
            TemperatureMeasureUnits::Fahrenheit => write!(f, "°F"),
            TemperatureMeasureUnits::Kelvin => write!(f, "K"),
            _ => std::fmt::Result::Err(std::fmt::Error), // "no implementation for provided value {:?}", &self
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Temperature {
    value: f32,
    measure_units: TemperatureMeasureUnits,
}

impl Temperature {
    pub fn new(value: f32, measure_units: TemperatureMeasureUnits) -> Temperature {
        Temperature {
            value,
            measure_units,
        }
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn get_measure_units(&self) -> TemperatureMeasureUnits {
        self.measure_units
    }

    /// # Examples
    /// ```
    /// use smart_house::temperature::TemperatureMeasureUnits;
    /// use smart_house::temperature::Temperature;
    /// let temperature = Temperature::new(10.0, smart_house::temperature::TemperatureMeasureUnits::Celsius);
    /// let converted_temperature = temperature.convert_from_to(smart_house::temperature::TemperatureMeasureUnits::Fahrenheit);
    /// assert_eq!(converted_temperature.get_value(), 50.0);
    /// assert_eq!(converted_temperature.get_measure_units(), TemperatureMeasureUnits::Fahrenheit);
    /// ```
    pub fn convert_from_to(&self, to: TemperatureMeasureUnits) -> Temperature {
        match (self.measure_units, to) {
            (TemperatureMeasureUnits::Celsius, TemperatureMeasureUnits::Fahrenheit) => {
                Temperature::new(self.value * 9.0 / 5.0 + 32.0, to)
            }
            (TemperatureMeasureUnits::Fahrenheit, TemperatureMeasureUnits::Celsius) => {
                Temperature::new((self.value - 32.0) * 5.0 / 9.0, to)
            }
            (TemperatureMeasureUnits::Celsius, TemperatureMeasureUnits::Kelvin) => {
                Temperature::new(self.value + 273.15, to)
            }
            (TemperatureMeasureUnits::Kelvin, TemperatureMeasureUnits::Celsius) => {
                Temperature::new(self.value - 273.15, to)
            }
            _ => *self,
        }
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.measure_units)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LightInfo {
    pub device_id: DeviceId,
    pub intensity: f32,
}
