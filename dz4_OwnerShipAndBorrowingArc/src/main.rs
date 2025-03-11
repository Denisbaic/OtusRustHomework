struct SmartSocket;

impl SmartSocket
{
    fn new() -> SmartSocket {
        SmartSocket
    }

    fn turn_on(&mut self) {
        todo!()
    }

    fn turn_off(&mut self) {
        todo!()
    }

    fn toggle(&mut self) {
        if self.is_on() { self.turn_off(); } else { self.turn_on(); }
    }

    fn is_on(&self) -> bool {
        todo!()
    }

    fn is_off(&self) -> bool {
        !self.is_on()
    }

    fn get_device_description(&self) -> &str {
        todo!()
    }

    fn get_current_power_consumption(&self) -> f32 {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
enum TemperatureMeasureUnits
{
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug, Clone, Copy)]
struct Temperature
{
    value: f32,
    measure_units: TemperatureMeasureUnits,
}

impl Temperature
{
    fn new(value: f32, measure_units: TemperatureMeasureUnits) -> Temperature {
        Temperature { value, measure_units }
    }

    fn convert_from_to(&self, to: TemperatureMeasureUnits) -> Temperature {
        match (self.measure_units, to) {
            (TemperatureMeasureUnits::Celsius, TemperatureMeasureUnits::Fahrenheit) => Temperature::new(self.value * 9.0 / 5.0 + 32.0, to),
            (TemperatureMeasureUnits::Fahrenheit, TemperatureMeasureUnits::Celsius) => Temperature::new((self.value - 32.0) * 5.0 / 9.0, to),
            (TemperatureMeasureUnits::Celsius, TemperatureMeasureUnits::Kelvin) => Temperature::new(self.value + 273.15, to),
            (TemperatureMeasureUnits::Kelvin, TemperatureMeasureUnits::Celsius) => Temperature::new(self.value - 273.15, to),
            _ => *self,
        }
    }
}

struct Thermometer;

impl Thermometer
{
    fn new() -> Thermometer {
        Thermometer
    }

    fn get_temperature(&self, _temperature_units: TemperatureMeasureUnits) -> Temperature {
        todo!()
    }
}

fn main() {
    let mut smart_socket = SmartSocket::new();
    smart_socket.is_on();
    smart_socket.is_off();
    smart_socket.toggle();
    let description = smart_socket.get_device_description();
    println!("{}", description);

    smart_socket.get_current_power_consumption();


    let initial_temperature = Temperature::new(0.0, TemperatureMeasureUnits::Celsius);
    let _converted_temperature = initial_temperature.convert_from_to(TemperatureMeasureUnits::Fahrenheit);
    let converted_temperature = initial_temperature.convert_from_to(TemperatureMeasureUnits::Kelvin);
    println!("{:?}", converted_temperature);

    let desired_temperature_unit = TemperatureMeasureUnits::Celsius;
    let thermometer = Thermometer::new();
    let current_temperature = thermometer.get_temperature(desired_temperature_unit);

    println!("{} {:?}", current_temperature.value, current_temperature.measure_units);
}
