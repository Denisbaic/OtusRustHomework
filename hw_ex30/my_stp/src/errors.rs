#[derive(Debug, thiserror::Error)]
pub enum ConnectError {
    #[error("bad handshake")]
    BadHandshake,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error(transparent)]
    Connect(#[from] ConnectError),
    #[error("failed to send request: {0}")]
    Send(#[from] SendError),
    #[error("failed to receive response: {0}")]
    Recv(#[from] RecvError),
}

#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RecvError {
    #[error("bad encoding")]
    BadEncoding,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
