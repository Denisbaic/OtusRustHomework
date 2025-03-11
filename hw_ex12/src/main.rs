use std::rc::Rc;

use smart_house::device::Device;
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

// custom reporters
struct OwningDeviceReportProvider<T>
where
    T: Device,
{
    socket: T,
}

impl<T: Device> Reporter for OwningDeviceReportProvider<T> {
    fn create_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.socket.create_report()
    }
}

struct BorrowingDeviceReportProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b Thermometer,
}

impl<'a, 'b> Reporter for BorrowingDeviceReportProvider<'a, 'b> {
    fn create_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(
            "{}\n{}",
            self.socket.create_report()?,
            self.thermo.create_report()?
        ))
    }
}

struct CustomReporter;

impl CustomReporter {
    fn create_report<T>(reporte_sources: &[&T]) -> String
    where
        T: Reporter + ?Sized,
    {
        let title = "===============Custom Report===============";

        let content = reporte_sources
            .iter()
            .map(|reporter| match reporter.create_report() {
                Ok(report) => report,
                Err(err) => format!("Error: {}", err),
            })
            .collect::<Vec<String>>()
            .join("\n");

        let end = "===============Custom Report End============";

        format!("{title}\n{content}\n{end}\n")
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

    let smart_house = smart_house::SmartHouse::new(vec![
        smart_house::Room::new(
            "Кухня".to_string(),
            vec![
                Box::new(Thermometer::new(
                    "Термометр1",
                    Rc::clone(&temperature_provider1) as Rc<dyn ThermometerInfoProvider>,
                )),
                Box::new(Thermometer::new(
                    "Термометр2",
                    Rc::clone(&temperature_provider1) as Rc<dyn ThermometerInfoProvider>,
                )),
                Box::new(SmartSocket::new(
                    "Розетка1",
                    Rc::clone(&energy_provider1) as Rc<dyn SmartSocketInfoProvider>,
                )),
                Box::new(SmartSocket::new(
                    "Розетка2",
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
                Box::new(Thermometer::new(
                    "Термометр4",
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
                Box::new(SmartSocket::new(
                    "Розетка5",
                    Rc::clone(&energy_provider3) as Rc<dyn SmartSocketInfoProvider>,
                )),
            ],
        ),
    ]);
    println!(
        "{}",
        smart_house
            .create_report()
            .expect("Failed to create report")
    );

    let custom_report1 = CustomReporter::create_report(smart_house.devices().as_slice());
    let custom_report2 = CustomReporter::create_report([&smart_house as &dyn Reporter].as_slice());
    println!("{custom_report1}");
    println!("{custom_report2}");

    let owning_device_report_provider = OwningDeviceReportProvider {
        socket: SmartSocket::new(
            "Розетка",
            Rc::clone(&energy_provider1) as Rc<dyn SmartSocketInfoProvider>,
        ),
    };

    let socket = SmartSocket::new(
        "Розетка6",
        Rc::clone(&energy_provider1) as Rc<dyn SmartSocketInfoProvider>,
    );
    let thermo = Thermometer::new(
        "Термометр6",
        Rc::clone(&temperature_provider1) as Rc<dyn ThermometerInfoProvider>,
    );
    let borrowing_device_report_provider = BorrowingDeviceReportProvider {
        socket: &socket,
        thermo: &thermo,
    };

    println!(
        "{}",
        CustomReporter::create_report([&owning_device_report_provider].as_slice())
    );
    println!(
        "{}",
        CustomReporter::create_report([&borrowing_device_report_provider].as_slice())
    );
    let test_slice: [&dyn Reporter; 2] = [
        &owning_device_report_provider,
        &borrowing_device_report_provider,
    ];
    println!("{}", CustomReporter::create_report(test_slice.as_slice()));
}
