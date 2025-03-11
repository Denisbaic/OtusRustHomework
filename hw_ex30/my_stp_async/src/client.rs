use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
};

use crate::{
    errors::{ConnectError, RequestError},
    recv_string, send_string,
};

pub struct StpClient;

impl StpClient {
    pub async fn connect<Addrs>(addrs: Addrs) -> Result<StpConnection, ConnectError>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpStream::connect(addrs).await?;
        Self::try_handshake(tcp).await
    }

    async fn try_handshake(mut stream: TcpStream) -> Result<StpConnection, ConnectError> {
        stream.write_all(b"clnt").await?;
        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf).await?;
        if &buf != b"serv" {
            return Err(ConnectError::BadHandshake);
        }
        Ok(StpConnection { stream })
    }
}

#[derive(Debug)]
pub struct StpConnection {
    stream: TcpStream,
}

impl StpConnection {
    pub async fn send_request<T>(&mut self, request: T) -> Result<String, RequestError>
    where
        T: ToString,
    {
        send_string(request.to_string().as_str(), &mut self.stream).await?;
        let response = recv_string(&mut self.stream).await?;
        Ok(response)
    }
}
