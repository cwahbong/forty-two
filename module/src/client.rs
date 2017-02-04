use common::EventCodec;
use types::Event;

use std::io;
use std::net;
use tokio_core;
use tokio_proto;
use tokio_proto::TcpClient;
use tokio_proto::pipeline::{Pipeline, ClientProto, ClientService};
use tokio_service::Service;

pub struct ModuleClientProto;

impl<T> ClientProto<T> for ModuleClientProto
    where T: tokio_core::io::Io + 'static
{
    type Request = Event;
    type Response = Event;
    type Transport = tokio_core::io::Framed<T, EventCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(EventCodec))
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

    pub fn request(&mut self, event: Event) -> Result<Event, io::Error> {
        self.core.run(self.service.call(event))
    }
}

pub fn connect(addr: &net::SocketAddr) -> Result<Client, io::Error> {
    Client::connect(addr)
}
