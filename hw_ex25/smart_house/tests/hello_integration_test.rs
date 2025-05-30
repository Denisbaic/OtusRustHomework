use smart_house::{
    smart_tools::{smart_socket::SmartSocketInfoProvider, thermomener::ThermometerInfoProvider},
    temperature::{Temperature, TemperatureMeasureUnits},
};

struct TemperatureProvider {
    value: f32,
    measure_units: TemperatureMeasureUnits,
}

impl ThermometerInfoProvider for TemperatureProvider {
    fn get_temperature(&self) -> Temperature {
        Temperature::new(self.value, self.measure_units)
    }
}

struct EnergyProvider {
    value: f32,
}

impl SmartSocketInfoProvider for EnergyProvider {
    fn get_current_power_consumption(&self) -> f32 {
        self.value
    }
}

#[cfg(test)]
mod hello_integration_test {
    #[test]
    fn add_rooms_to_smart_house() -> Result<(), Box<dyn std::error::Error>> {
        /*
        let energy_provider1 = Arc::new(EnergyProvider { value: 100.0 });
        let temperature_provider1 = Arc::new(TemperatureProvider {
            value: 16.0,
            measure_units: TemperatureMeasureUnits::Celsius,
        });

        let energy_provider2 = Arc::new(EnergyProvider { value: 50.0 });
        let temperature_provider2 = Arc::new(TemperatureProvider {
            value: 15.0,
            measure_units: TemperatureMeasureUnits::Celsius,
        });

        let energy_provider3 = Arc::new(EnergyProvider { value: 30.0 });
        let temperature_provider3 = Arc::new(TemperatureProvider {
            value: 14.0,
            measure_units: TemperatureMeasureUnits::Celsius,
        });

        let mut thermometer = Box::new(Thermometer::new(
            "Термометр1",
            Arc::clone(&temperature_provider1) as Arc<dyn ThermometerInfoProvider>,
        ));
        thermometer.turn_off();
        let smart_house = smart_house::SmartHouse::new(vec![
            smart_house::Room::new(
                "Кухня".to_string(),
                vec![
                    thermometer,
                    Box::new(Thermometer::new(
                        "Термометр2",
                        Arc::clone(&temperature_provider1) as Arc<dyn ThermometerInfoProvider>,
                    )),
                    Box::new(SmartSocket::new(
                        "Розетка1",
                        Arc::clone(&energy_provider1) as Arc<dyn SmartSocketInfoProvider>,
                    )),
                    Box::new(SmartSocket::new(
                        "Розетка2",
                        Arc::clone(&energy_provider1) as Arc<dyn SmartSocketInfoProvider>,
                    )),
                ],
            ),
            smart_house::Room::new(
                "Спальня".to_string(),
                vec![
                    Box::new(Thermometer::new(
                        "Термометр3",
                        Arc::clone(&temperature_provider2) as Arc<dyn ThermometerInfoProvider>,
                    )),
                    Box::new(Thermometer::new(
                        "Термометр4",
                        Arc::clone(&temperature_provider2) as Arc<dyn ThermometerInfoProvider>,
                    )),
                    Box::new(SmartSocket::new(
                        "Розетка3",
                        Arc::clone(&energy_provider2) as Arc<dyn SmartSocketInfoProvider>,
                    )),
                ],
            ),
            smart_house::Room::new(
                "Гостиная".to_string(),
                vec![
                    Box::new(Thermometer::new(
                        "Термометр5",
                        Arc::clone(&temperature_provider3) as Arc<dyn ThermometerInfoProvider>,
                    )),
                    Box::new(SmartSocket::new(
                        "Розетка4",
                        Arc::clone(&energy_provider3) as Arc<dyn SmartSocketInfoProvider>,
                    )),
                    Box::new(SmartSocket::new(
                        "Розетка5",
                        Arc::clone(&energy_provider3) as Arc<dyn SmartSocketInfoProvider>,
                    )),
                ],
            ),
        ]);
        assert!(smart_house.create_report().is_err());
        */
        Ok(())
    }
}
