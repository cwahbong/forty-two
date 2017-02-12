use api::{Api, NewApi};
use common::SizedJsonCodec;

use futures::{future, BoxFuture, Future};
use std::io;
use std::net;
use serde_json;
use tokio_core;
use tokio_proto::TcpServer;
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;

struct ModuleServerProto;

impl<T> ServerProto<T> for ModuleServerProto
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

struct ModuleService<NA: NewApi> {
    api: NA::Api
}

impl<NA: NewApi> ModuleService<NA> {
    pub fn new() -> Self {
        ModuleService { api: NA::new_api() }
    }
}

impl<NA: NewApi> Service for ModuleService<NA> {
    type Request = serde_json::Value;
    type Response = serde_json::Value;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, request: Self::Request) -> Self::Future {
        warn!("CALL");
        let request_event = serde_json::from_value(request).unwrap(); // XXX need error handling instead of unwrap
        let response_event = self.api.run(request_event);
        future::ok(serde_json::to_value(response_event).unwrap()).boxed()
    }
}

pub fn serve<NA: NewApi>(addr: net::SocketAddr) {
    TcpServer::new(ModuleServerProto, addr).serve(|| Ok(ModuleService::<NA>::new()))
}
