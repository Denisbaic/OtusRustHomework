use std::io;

#[derive(Debug)]
pub enum SmartHouseInitError {
    LoadingError(io::Error),
}

impl From<io::Error> for SmartHouseInitError {
    fn from(value: io::Error) -> Self {
        SmartHouseInitError::LoadingError(value)
    }
}

#[derive(Debug)]
pub enum CreateNewServerError {
    SmartHouseInitError(SmartHouseInitError),
    Io(io::Error),
}

impl From<SmartHouseInitError> for CreateNewServerError {
    fn from(value: SmartHouseInitError) -> Self {
        CreateNewServerError::SmartHouseInitError(value)
    }
}

impl From<io::Error> for CreateNewServerError {
    fn from(value: io::Error) -> Self {
        CreateNewServerError::Io(value)
    }
}

#[derive(Debug)]
pub enum ProccessRequestError {
    CantProccessRequest,
    ProccessorError(ProccessorError),
}

impl From<ProccessorError> for ProccessRequestError {
    fn from(value: ProccessorError) -> Self {
        ProccessRequestError::ProccessorError(value)
    }
}

#[derive(Debug)]
pub enum ProccessorError {
    CantProccessRequest,
    BadRequestParam,
    CantGetReport,
    CantFindRoom,
    CantFindDevice,
}
