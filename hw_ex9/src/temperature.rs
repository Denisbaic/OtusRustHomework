use std::fmt::Display;

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
        write!(f, "{}{}", self.value, self.measure_units)
    }
}

#[cfg(test)]
mod temperature_tests {
    use super::*;

    #[test]
    fn init_temperature() {
        let _test_value = Temperature::new(10.0, TemperatureMeasureUnits::Celsius);
    }

    #[test]
    fn convert_temperature_from_celsius_to_fahrenheit() {
        let test_value = Temperature::new(10.0, TemperatureMeasureUnits::Celsius);
        let converted_value = test_value.convert_from_to(TemperatureMeasureUnits::Fahrenheit);
        assert_eq!(converted_value.value, 50.0);
        assert_eq!(
            converted_value.measure_units,
            TemperatureMeasureUnits::Fahrenheit
        );
    }

    #[test]
    fn convert_temperature_from_celsius_to_kelvin() {
        let test_value = Temperature::new(10.0, TemperatureMeasureUnits::Celsius);
        let converted_value = test_value.convert_from_to(TemperatureMeasureUnits::Kelvin);
        assert_eq!(converted_value.value, 283.15);
        assert_eq!(
            converted_value.measure_units,
            TemperatureMeasureUnits::Kelvin
        );
    }

    #[test]
    fn convert_temperature_from_celsius_to_celsius() {
        let test_value = Temperature::new(10.0, TemperatureMeasureUnits::Celsius);
        let converted_value = test_value.convert_from_to(TemperatureMeasureUnits::Celsius);
        assert_eq!(converted_value.value, 10.0);
        assert_eq!(
            converted_value.measure_units,
            TemperatureMeasureUnits::Celsius
        );
    }

    #[test]
    fn test_celsius_temperature_measure_units_display() {
        let units = TemperatureMeasureUnits::Celsius;
        assert_eq!(format!("{}", units), "°C");
    }

    #[test]
    fn test_temperature_display_celsius() {
        let temp = Temperature::new(10.0, TemperatureMeasureUnits::Celsius);
        assert_eq!(format!("{}", temp), "10°C");
    }

    #[test]
    fn test_temperature_get_value() {
        let temp = Temperature::new(10.0, TemperatureMeasureUnits::Celsius);
        assert_eq!(temp.value, 10.0);
    }

    #[test]
    fn test_temperature_get_measure_units() {
        let temp = Temperature::new(10.0, TemperatureMeasureUnits::Celsius);
        assert_eq!(temp.measure_units, TemperatureMeasureUnits::Celsius);
    }
}
