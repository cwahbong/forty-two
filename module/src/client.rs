use common::SizedJsonCodec;
use types::{RequestEvent, ResponseEvent};

use std::io;
use std::net;
use serde_json;
use tokio_core;
use tokio_proto;
use tokio_proto::TcpClient;
use tokio_proto::pipeline::{Pipeline, ClientProto, ClientService};
use tokio_service::Service;

pub struct ModuleClientProto;

impl<T> ClientProto<T> for ModuleClientProto
    where T: tokio_core::io::Io + 'static
{
    type Request = serde_json::Value;
    type Response = serde_json::Value;
    type Transport = tokio_core::io::Framed<T, SizedJsonCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(SizedJsonCodec))
    }
}

pub type Connect = tokio_proto::Connect<Pipeline, ModuleClientProto>;

pub struct Client {
    core: tokio_core::reactor::Core,
    service: ClientService<tokio_core::net::TcpStream, ModuleClientProto>,
}

impl Client {
    fn connect(addr: &net::SocketAddr) -> Result<Client, io::Error> {
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();
        let connect_fut = TcpClient::new(ModuleClientProto).connect(addr, &handle);
        let service = core.run(connect_fut)?;
        Ok(Client {
            core: core,
            service: service,
        })
    }

    pub fn request(&mut self, request_event: RequestEvent) -> Result<ResponseEvent, io::Error> {
        // XXX error handling instead of unwrap
        let request = serde_json::to_value(request_event).unwrap(); // XXX need error handling instead of unwrap
        let response = self.core.run(self.service.call(request))?;
        Ok(serde_json::from_value(response).unwrap())
    }
}

pub fn connect(addr: &net::SocketAddr) -> Result<Client, io::Error> {
    Client::connect(addr)
}
