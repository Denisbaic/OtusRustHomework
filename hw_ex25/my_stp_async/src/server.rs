use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
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
    pub async fn bind<Addrs>(addrs: Addrs) -> io::Result<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs).await?;
        Ok(Self { tcp })
    }

    pub async fn accept(&self) -> Result<StpConnection, ConnectError> {
        let (stream, _) = self.tcp.accept().await?;
        Self::try_handshake(stream).await
    }

    async fn try_handshake(mut stream: TcpStream) -> Result<StpConnection, ConnectError> {
        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf).await?;
        if &buf != b"clnt" {
            return Err(ConnectError::BadHandshake);
        }
        stream.write_all(b"serv").await?;
        Ok(StpConnection { stream })
    }
}

#[derive(Debug)]
pub struct StpConnection {
    stream: TcpStream,
}

impl StpConnection {
    pub async fn proccess_request<F>(mut self, handler: F) -> Result<(), RequestError>
    where
        F: FnOnce(String) -> String,
    {
        let request = recv_string(&mut self.stream).await?;
        let response = handler(request);
        send_string(response, &mut self.stream).await?;
        Ok(())
    }
}
