use common::EventCodec;
use types::Event;

use futures::{future, BoxFuture, Future};
use std::io;
use std::net;
use tokio_core;
use tokio_proto::TcpServer;
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;

struct ModuleServerProto;

impl<T> ServerProto<T> for ModuleServerProto
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

struct ModuleService;

impl Service for ModuleService {
    type Request = Event;
    type Response = Event;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        warn!("CALL");
        future::ok(req).boxed() // TODO currently just echo
    }
}

pub fn serve(addr: net::SocketAddr) {
    TcpServer::new(ModuleServerProto, addr).serve(|| Ok(ModuleService))
}
