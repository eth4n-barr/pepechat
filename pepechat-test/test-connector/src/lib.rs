use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use pepechat::connection::{Connection, SetupHooks};
use pepechat::message::Message;
use async_trait::async_trait;
use tokio::net::{TcpListener, TcpSocket, TcpStream};
use crate::TestConnectorError::CouldntGetSetupData;

pub struct Connector {
    port: i32,
    other_ip: SocketAddr,
    sender: Option<TcpStream>,
    listener: Option<TcpListener>
}

enum TestConnectorError {
    CouldntGetSetupData,
}

#[async_trait]
impl Connection<TestConnectorError> for Connector {
    fn setup(&mut self, hooks: &SetupHooks) -> Result<(), TestConnectorError> {
        let port = (hooks.get_int)("What port should I listen on?".parse().unwrap());
        let other_ip = (hooks.get_string)("What IP is the other peer listening on?".parse().unwrap());

        return if port.is_err() || other_ip.is_err() {
            Err(CouldntGetSetupData)
        } else {
            self.other_ip = other_ip.unwrap().parse().unwrap();
            self.port = port.unwrap();
            Ok(())
        }
    }

    async fn connect(&mut self) {
        if self.sender.is_some() || self.listener.is_some() {
            self.disconnect();
        }
        
        self.sender = Some(TcpStream::connect(self.other_ip).await.unwrap());
        let mut listener = TcpSocket::new_v4().unwrap();
        listener.bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), self.port as u16));
        self.listener = Some(listener.listen(128).unwrap());
    }

    async fn disconnect(&mut self) {
        self.sender = None;
        self.listener = None;
    }

    async fn send_message(&mut self, message: Message) -> Result<(), TestConnectorError> {
        todo!()
    }
}