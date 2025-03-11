use std::io;

#[derive(Debug)]
pub enum ConnectError {
    BadHandshake,
    Io(io::Error),
}

impl From<io::Error> for ConnectError {
    fn from(value: io::Error) -> Self {
        ConnectError::Io(value)
    }
}

#[derive(Debug)]
pub enum RequestError {
    Connect(ConnectError),
    Send(SendError),
    Recv(RecvError),
}

impl From<ConnectError> for RequestError {
    fn from(value: ConnectError) -> Self {
        RequestError::Connect(value)
    }
}

impl From<SendError> for RequestError {
    fn from(value: SendError) -> Self {
        RequestError::Send(value)
    }
}

impl From<RecvError> for RequestError {
    fn from(value: RecvError) -> Self {
        RequestError::Recv(value)
    }
}

#[derive(Debug)]
pub enum SendError {
    Io(io::Error),
}

impl From<io::Error> for SendError {
    fn from(value: io::Error) -> Self {
        SendError::Io(value)
    }
}

#[derive(Debug)]
pub enum RecvError {
    BadEncoding,
    Io(io::Error),
}

impl From<io::Error> for RecvError {
    fn from(value: io::Error) -> Self {
        RecvError::Io(value)
    }
}
