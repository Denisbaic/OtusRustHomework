use std::sync::Arc;
use std::{net::ToSocketAddrs, rc::Rc};

use errors::{CreateNewServerError, ProccessRequestError, ProccessorError, SmartHouseInitError};
use processors::{
    DeviceListProcessor, DeviceReportProcessor, HelloProcessor, IsDeviceOnProcessor,
    RequestProcessor, RoomsListProcessor, SetDevicePowerStateProcessor,
};
use smart_house::{smart_tools, temperature, SmartHouse};
use smart_tools::smart_socket::{SmartSocket, SmartSocketInfoProvider, TemperatureProvider};
use smart_tools::thermomener::{EnergyProvider, Thermometer, ThermometerInfoProvider};
use temperature::TemperatureMeasureUnits;

pub mod errors;
mod processors;

pub struct SmartHouseServer {
    smart_house: smart_house::SmartHouse,
    stp: my_stp::server::StpServer,
    processors: Vec<Arc<dyn RequestProcessor>>,
}

impl SmartHouseServer {
    fn get_processors() -> Vec<Arc<dyn RequestProcessor>> {
        let processors: Vec<Arc<dyn RequestProcessor>> = vec![
            Arc::new(HelloProcessor),
            Arc::new(RoomsListProcessor),
            Arc::new(DeviceListProcessor),
            Arc::new(DeviceReportProcessor),
            Arc::new(SetDevicePowerStateProcessor),
            Arc::new(IsDeviceOnProcessor),
        ];
        processors
    }

    fn init_smart_house() -> Result<SmartHouse, SmartHouseInitError> {
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
        Ok(smart_house)
    }

    pub fn new<Addrs>(addrs: Addrs) -> Result<Self, CreateNewServerError>
    where
        Addrs: ToSocketAddrs,
    {
        Ok(SmartHouseServer {
            smart_house: SmartHouseServer::init_smart_house()?,
            stp: my_stp::server::StpServer::bind(addrs)?,
            processors: SmartHouseServer::get_processors(),
        })
    }

    pub fn start_server_listening(&mut self) {
        println!("Starting server...");

        loop {
            let connection = self.stp.accept();
            if connection.is_err() {
                continue;
            }

            let success_connection = connection.unwrap();
            let proccess_result =
                success_connection.proccess_request(|reqest| match self.process_request(reqest) {
                    Ok(response) => response,
                    Err(e) => {
                        eprintln!("Proccess request error : {:?}", e);
                        format!("Proccess request error : {:?}", e)
                    }
                });

            if let Err(request_error) = proccess_result {
                eprintln!("Request error : {:?}", request_error);
            }
        }
    }

    fn process_request(&mut self, request: String) -> Result<String, ProccessRequestError> {
        for proccessor in &mut self.processors {
            let result = proccessor.try_process(&request, &mut self.smart_house);
            match result {
                Err(ProccessorError::CantProccessRequest) => continue,
                Err(e) => return Err(ProccessRequestError::ProccessorError(e)),
                _ => (),
            }

            if let Ok(response) = result {
                return Ok(response);
            }
        }
        Err(ProccessRequestError::CantProccessRequest)
    }
}
