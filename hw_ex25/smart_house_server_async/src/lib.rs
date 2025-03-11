use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};

use errors::{CreateNewServerError, ProccessRequestError, ProccessorError, SmartHouseInitError};
use processors::{
    CancelDeviceReportStreamProcessor, DeviceListProcessor, DeviceReportProcessor,
    GetDeviceReportStreamProcessor, HelloProcessor, IsDeviceOnProcessor, RequestProcessor,
    RoomsListProcessor, SetDevicePowerStateProcessor,
};
use smart_house::{smart_tools, temperature, SmartHouse};
use smart_tools::smart_socket::{SmartSocket, SmartSocketInfoProvider, TemperatureProvider};
use smart_tools::thermomener::{EnergyProvider, Thermometer, ThermometerInfoProvider};
use temperature::TemperatureMeasureUnits;
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::sync::watch::{self, Sender};
use tokio::sync::Mutex;

pub mod errors;
mod processors;

struct ServerStore {
    execution_threads: HashMap<String, Sender<bool>>,
    message_thread: Option<Sender<bool>>,
    udp_socket: UdpSocket,
}

pub struct SmartHouseServer {
    smart_house: Arc<tokio::sync::Mutex<SmartHouse>>,
    stp: Arc<Mutex<my_stp_async::server::StpServer>>,
    processors: Arc<Vec<Box<dyn RequestProcessor>>>,
    server_threads: Arc<tokio::sync::Mutex<ServerStore>>,
}

impl SmartHouseServer {
    fn create_processors() -> Vec<Box<dyn RequestProcessor>> {
        let processors: Vec<Box<dyn RequestProcessor>> = vec![
            Box::new(HelloProcessor),
            Box::new(RoomsListProcessor),
            Box::new(DeviceListProcessor),
            Box::new(IsDeviceOnProcessor),
            Box::new(DeviceReportProcessor),
            Box::new(GetDeviceReportStreamProcessor),
            Box::new(SetDevicePowerStateProcessor),
            Box::new(CancelDeviceReportStreamProcessor),
        ];
        processors
    }

    fn init_smart_house() -> Result<SmartHouse, SmartHouseInitError> {
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

        let smart_house = smart_house::SmartHouse::new(vec![
            smart_house::Room::new(
                "Кухня".to_string(),
                vec![
                    Arc::new(RwLock::new(Box::new(Thermometer::new(
                        "Термометр1",
                        temperature_provider1.clone(),
                    )))),
                    Arc::new(RwLock::new(Box::new(SmartSocket::new(
                        "Розетка1",
                        Arc::clone(&energy_provider1) as Arc<dyn SmartSocketInfoProvider>,
                    )))),
                ],
            ),
            smart_house::Room::new(
                "Спальня".to_string(),
                vec![
                    Arc::new(RwLock::new(Box::new(Thermometer::new(
                        "Термометр3",
                        Arc::clone(&temperature_provider2) as Arc<dyn ThermometerInfoProvider>,
                    )))),
                    Arc::new(RwLock::new(Box::new(SmartSocket::new(
                        "Розетка3",
                        Arc::clone(&energy_provider2) as Arc<dyn SmartSocketInfoProvider>,
                    )))),
                ],
            ),
            smart_house::Room::new(
                "Гостиная".to_string(),
                vec![
                    Arc::new(RwLock::new(Box::new(Thermometer::new(
                        "Термометр5",
                        Arc::clone(&temperature_provider3) as Arc<dyn ThermometerInfoProvider>,
                    )))),
                    Arc::new(RwLock::new(Box::new(SmartSocket::new(
                        "Розетка4",
                        Arc::clone(&energy_provider3) as Arc<dyn SmartSocketInfoProvider>,
                    )))),
                ],
            ),
        ]);
        Ok(smart_house)
    }

    pub async fn new<Addrs>(tcp_addr: Addrs, udp_addr: Addrs) -> Result<Self, CreateNewServerError>
    where
        Addrs: ToSocketAddrs,
    {
        Ok(SmartHouseServer {
            smart_house: Arc::new(tokio::sync::Mutex::new(
                SmartHouseServer::init_smart_house()?
            )),
            stp: Arc::new(tokio::sync::Mutex::new(
                my_stp_async::server::StpServer::bind(tcp_addr).await?,
            )),
            processors: Arc::new(SmartHouseServer::create_processors()),
            server_threads: Arc::new(Mutex::new(ServerStore {
                execution_threads: HashMap::new(),
                message_thread: None,
                udp_socket: UdpSocket::bind(udp_addr).await?,
            })),
        })
    }

    pub fn start_server_listening(&mut self) {
        println!("Starting server...");

        let server_threads_ptr = self.server_threads.clone();
        let smart_house_ptr = self.smart_house.clone();
        let processors_ptr = self.processors.clone();
        let stp_atomic = self.stp.clone();

        let (canceller, cancellation_token) = watch::channel(false);

        tokio::spawn(async move {
            let server_threads_ptr = server_threads_ptr;
            let smart_house_ptr = smart_house_ptr;
            let processors_ptr = processors_ptr;
            let stp_atomic = stp_atomic;

            loop {
                if *cancellation_token.borrow() {
                    break;
                }

                let connection = stp_atomic.lock().await.accept().await;
                if connection.is_err() {
                    continue;
                }
                let success_connection = connection.unwrap();

                let server_threads = server_threads_ptr.clone();
                let mut smart_house = smart_house_ptr.lock().await;

                let processors = processors_ptr.as_ref();

                let proccess_result = success_connection
                    .proccess_request(move |reqest| {
                        match Self::process_request_by_processors(
                            reqest,
                            server_threads,
                            smart_house.deref_mut(),
                            processors,
                        ) {
                            Ok(response) => response,
                            Err(e) => {
                                eprintln!("Proccess request error : {:?}", e);
                                format!("Proccess request error : {:?}", e)
                            }
                        }
                    })
                    .await;

                if let Err(request_error) = proccess_result {
                    eprintln!("Request error : {:?}", request_error);
                }
            }
        });
        let server_threads = self.server_threads.clone();
        tokio::task::spawn_blocking(move || {
            server_threads.blocking_lock().message_thread = Some(canceller);
        });
    }

    fn process_request_by_processors(
        request: String,
        server: Arc<Mutex<ServerStore>>,
        smart_house: &mut SmartHouse,
        processors: &Vec<Box<dyn RequestProcessor>>,
    ) -> Result<String, ProccessRequestError> {
        for proccessor in processors.iter() {
            let result = proccessor.try_process(&request, server.clone(), smart_house);
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

impl Drop for SmartHouseServer {
    fn drop(&mut self) {
        println!("Dropping server...");
        println!("join all server threads");

        let mut write_guard = self.server_threads.blocking_lock();

        if let Some(thread) = write_guard.message_thread.take() {
            thread.send(true).unwrap();
            println!("tcp message thread joined");
        }

        println!("join udp server threads");
        let execution_threads = std::mem::take(&mut write_guard.execution_threads);

        for (_, value) in execution_threads.into_iter() {
            value.send(true).unwrap();
        }

        println!("udp server threads joined");

        println!("server dropped");
    }
}
