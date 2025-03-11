#[derive(Debug, thiserror::Error)]
pub enum SmartHouseInitError {
    #[error(transparent)]
    LoadingError(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CreateNewServerError {
    #[error(transparent)]
    SmartHouseInitError(#[from] SmartHouseInitError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ProccessRequestError {
    #[error("Cant read smart house")]
    CantReadSmartHouse,
    #[error("Cant proccess request")]
    CantProccessRequest,
    #[error(transparent)]
    ProccessorError(#[from] ProccessorError),
}

#[derive(Debug, thiserror::Error)]
pub enum ProccessorError {
    #[error("Cant proccess request")]
    CantProccessRequest,
    #[error("Bad request param")]
    BadRequestParam,
    #[error("Cant get report")]
    CantGetReport,
    #[error("Cant find room")]
    CantFindRoom,
    #[error("Cant find device")]
    CantFindDevice,
}
