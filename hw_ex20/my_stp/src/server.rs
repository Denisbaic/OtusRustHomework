use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use crate::{
    errors::{ConnectError, RequestError},
    recv_string, send_string,
};

pub struct StpServer {
    tcp: TcpListener,
}

impl StpServer {
    pub fn bind<Addrs>(addrs: Addrs) -> io::Result<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs)?;
        Ok(Self { tcp })
    }

    pub fn accept(&self) -> Result<StpConnection, ConnectError> {
        let (stream, _) = self.tcp.accept()?;
        Self::try_handshake(stream)
    }

    fn try_handshake(mut stream: TcpStream) -> Result<StpConnection, ConnectError> {
        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf)?;
        if &buf != b"clnt" {
            return Err(ConnectError::BadHandshake);
        }
        stream.write_all(b"serv")?;
        Ok(StpConnection { stream })
    }
}

#[derive(Debug)]
pub struct StpConnection {
    stream: TcpStream,
}

impl StpConnection {
    pub fn proccess_request<F>(mut self, handler: F) -> Result<(), RequestError>
    where
        F: FnOnce(String) -> String,
    {
        let request = recv_string(&mut self.stream)?;
        let response = handler(request);
        send_string(response, &mut self.stream)?;
        Ok(())
    }
}
