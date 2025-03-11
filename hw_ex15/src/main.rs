use std::rc::Rc;

use smart_house::reporter::Reporter;
use smart_house::smart_tools::{
    smart_socket::{SmartSocket, SmartSocketInfoProvider},
    thermomener::{Thermometer, ThermometerInfoProvider},
};
use smart_house::temperature::{Temperature, TemperatureMeasureUnits};

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

fn main() {
    let energy_provider1 = Rc::new(EnergyProvider { value: 100.0 });
    let temperature_provider1 = Rc::new(TemperatureProvider {
        value: 16.0,
        measure_units: TemperatureMeasureUnits::Celsius,
    });

    let energy_provider2 = Rc::new(EnergyProvider { value: 50.0 });
    let temperature_provider2 = Rc::new(TemperatureProvider {
        value: 15.0,
        measure_units: TemperatureMeasureUnits::Celsius,
    });

    let energy_provider3 = Rc::new(EnergyProvider { value: 30.0 });
    let temperature_provider3 = Rc::new(TemperatureProvider {
        value: 14.0,
        measure_units: TemperatureMeasureUnits::Celsius,
    });

    let mut smart_house = smart_house::SmartHouse::new(vec![
        smart_house::Room::new(
            "Кухня".to_string(),
            vec![
                Box::new(Thermometer::new(
                    "Термометр1",
                    Rc::clone(&temperature_provider1) as Rc<dyn ThermometerInfoProvider>,
                )),
                Box::new(SmartSocket::new(
                    "Розетка1",
                    Rc::clone(&energy_provider1) as Rc<dyn SmartSocketInfoProvider>,
                )),
            ],
        ),
        smart_house::Room::new(
            "Спальня".to_string(),
            vec![
                Box::new(Thermometer::new(
                    "Термометр3",
                    Rc::clone(&temperature_provider2) as Rc<dyn ThermometerInfoProvider>,
                )),
                Box::new(SmartSocket::new(
                    "Розетка3",
                    Rc::clone(&energy_provider2) as Rc<dyn SmartSocketInfoProvider>,
                )),
            ],
        ),
        smart_house::Room::new(
            "Гостиная".to_string(),
            vec![
                Box::new(Thermometer::new(
                    "Термометр5",
                    Rc::clone(&temperature_provider3) as Rc<dyn ThermometerInfoProvider>,
                )),
                Box::new(SmartSocket::new(
                    "Розетка4",
                    Rc::clone(&energy_provider3) as Rc<dyn SmartSocketInfoProvider>,
                )),
            ],
        ),
    ]);

    let room = smart_house.get_room_mut("");
    room.unwrap().add_unique_device(SmartSocket::new(
        "Розетка4",
        Rc::clone(&energy_provider3) as Rc<dyn SmartSocketInfoProvider>,
    ));

    let mut room = smart_house::Room::new(
        "Гостиная".to_string(),
        vec![
            Box::new(Thermometer::new(
                "Термометр5",
                Rc::clone(&temperature_provider3) as Rc<dyn ThermometerInfoProvider>,
            )),
            Box::new(SmartSocket::new(
                "Розетка4",
                Rc::clone(&energy_provider3) as Rc<dyn SmartSocketInfoProvider>,
            )),
            Box::new(SmartSocket::new(
                "Розетка5",
                Rc::clone(&energy_provider3) as Rc<dyn SmartSocketInfoProvider>,
            )),
        ],
    );
    room.add_unique_device(SmartSocket::new(
        "Розетка6",
        Rc::clone(&energy_provider3) as Rc<dyn SmartSocketInfoProvider>,
    ));
    smart_house.add_unique_room(room);

    println!(
        "{}",
        smart_house
            .create_report()
            .expect("Failed to create report")
    );
}
