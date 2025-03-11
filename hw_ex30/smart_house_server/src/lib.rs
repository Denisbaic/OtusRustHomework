use std::collections::HashMap;
use std::net::{ToSocketAddrs, UdpSocket};
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};
use std::thread;

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
use thread_cancellation_token::Canceller;

pub mod errors;
mod processors;

struct ServerStore {
    execution_threads: HashMap<String, Canceller>,
    message_thread: Option<Canceller>,
    udp_socket: UdpSocket,
}

pub struct SmartHouseServer {
    smart_house: Arc<RwLock<smart_house::SmartHouse>>,
    stp: Arc<my_stp::server::StpServer>,
    processors: Arc<Vec<Arc<dyn RequestProcessor>>>,
    server_threads: Arc<RwLock<ServerStore>>,
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
            Arc::new(GetDeviceReportStreamProcessor),
            Arc::new(CancelDeviceReportStreamProcessor),
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

        //Arc<RwLock<Box<(dyn Device)>>>
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

    pub fn new<Addrs>(tcp_addr: Addrs, udp_addr: Addrs) -> Result<Self, CreateNewServerError>
    where
        Addrs: ToSocketAddrs,
    {
        Ok(SmartHouseServer {
            smart_house: Arc::new(RwLock::new(SmartHouseServer::init_smart_house()?)),
            stp: Arc::new(my_stp::server::StpServer::bind(tcp_addr)?),
            processors: Arc::new(SmartHouseServer::get_processors()),
            server_threads: Arc::new(RwLock::new(ServerStore {
                execution_threads: HashMap::new(),
                message_thread: None,
                udp_socket: UdpSocket::bind(udp_addr)?,
            })),
        })
    }

    pub fn start_server_listening(&mut self) {
        println!("Starting server...");

        let smart_house_ptr = self.smart_house.clone();
        let processors_ptr = self.processors.clone();
        let stp_atomic = self.stp.clone();

        let server_threads_ptr = self.server_threads.clone();

        let (canceler, cancellation_token) = thread_cancellation_token::cancellation_token();

        let _: thread::JoinHandle<_> = thread::spawn(move || {
            let smart_house_ptr = smart_house_ptr;
            let processors_ptr = processors_ptr;
            let server_threads_ptr = server_threads_ptr;

            loop {
                if cancellation_token.should_cancel() {
                    break;
                }
                let connection = stp_atomic.accept();
                if connection.is_err() {
                    continue;
                }
                let success_connection = connection.unwrap();

                let proccess_result = success_connection.proccess_request(|reqest| {
                    match Self::process_request(
                        reqest,
                        server_threads_ptr.clone(),
                        &smart_house_ptr,
                        &processors_ptr,
                    ) {
                        Ok(response) => response,
                        Err(e) => {
                            eprintln!("Proccess request error : {:?}", e);
                            format!("Proccess request error : {:?}", e)
                        }
                    }
                });

                if let Err(request_error) = proccess_result {
                    eprintln!("Request error : {:?}", request_error);
                }
            }
        });

        self.server_threads.write().unwrap().message_thread = Some(canceler);
    }

    fn process_request(
        request: String,
        server: Arc<RwLock<ServerStore>>,
        smart_house_ptr: &RwLock<SmartHouse>,
        processors: &[Arc<dyn RequestProcessor>],
    ) -> Result<String, ProccessRequestError> {
        let lock_guard = smart_house_ptr.write();
        let mut lock_result = lock_guard.map_err(|_| ProccessRequestError::CantReadSmartHouse)?;
        let smart_house_ref = lock_result.deref_mut();

        for proccessor in processors.iter() {
            let result = proccessor.try_process(&request, server.clone(), smart_house_ref);
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

        let mut write_guard = self.server_threads.write().unwrap();

        if let Some(thread) = write_guard.message_thread.take() {
            thread.cancel();
            println!("tcp message thread joined");
        }

        println!("join udp server threads");
        let execution_threads = std::mem::take(&mut write_guard.execution_threads);

        for (_, value) in execution_threads.into_iter() {
            value.cancel();
        }

        println!("udp server threads joined");

        println!("server dropped");
    }
}
