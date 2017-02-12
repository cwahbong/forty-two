extern crate fte_module;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use fte_module::types::{Arguments, Data, RequestEvent, ResponseEvent};
use std::collections::hash_map::{Entry, HashMap};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

pub mod result;

pub use result::Error;
pub use result::Result;

#[derive(Serialize, Deserialize, Debug)]
struct RegistArgs {
    pub name: String,
    pub addr: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct QueryArgs { pub name: String }

#[derive(Serialize, Deserialize, Debug)]
struct DeregistArgs { pub name: String }

struct Inner {
    map: HashMap<String, SocketAddr>
}

impl Inner {
    pub fn new() -> Self {
        Inner { map: HashMap::new() }
    }

    pub fn regist(&mut self, args: RegistArgs) -> Result<()> {
        match self.map.entry(args.name) {
            Entry::Occupied(_) => Err(Error::AlreadyRegisted),
            Entry::Vacant(entry) => {
                entry.insert(SocketAddr::new(args.addr.parse()?, args.port));
                Ok(())
            }
        }
    }

    pub fn query(&self, args: QueryArgs) -> Result<bool> {
        Ok(self.map.contains_key(&args.name))
    }

    pub fn deregist(&mut self, args: DeregistArgs) -> Result<()> {
        self.map.remove(&args.name);
        Ok(())
    }
}

pub struct Modules {
    inner: Arc<RwLock<Inner>>
}

impl Modules {
    fn regist(&self, arguments: Arguments) -> Result<Data> {
        let mut acquired = self.inner.write()?;
        acquired.regist(serde_json::from_value(arguments)?).and_then(|v| Ok(serde_json::to_value(v)?))
    }

    fn query(&self, arguments: Arguments) -> Result<Data> {
        let acquired = self.inner.read()?;
        acquired.query(serde_json::from_value(arguments)?).and_then(|v| Ok(serde_json::to_value(v)?))
    }

    fn deregist(&self, arguments: Arguments) -> Result<Data> {
        let mut acquired = self.inner.write()?;
        acquired.deregist(serde_json::from_value(arguments)?).and_then(|v| Ok(serde_json::to_value(v)?))
    }
}

impl fte_module::api::Api for Modules {
    fn run(&self, request: RequestEvent) -> ResponseEvent {
        let data = match request.name.as_str() {
            "regist" => self.regist(request.arguments),
            "query" => self.query(request.arguments),
            "deregist" => self.deregist(request.arguments),
            _ => Err(Error::InvalidApi),
        };
        match data {
            Ok(data) => ResponseEvent {
                success: true,
                data: data,
            },
            Err(error) => ResponseEvent {
                success: false,
                data: serde_json::to_value(error).unwrap(),
            }
        }
    }
}

impl fte_module::api::NewApi for Modules {
    type Api = Self;
    fn new_api() -> Self::Api {
        Modules {
            inner: Arc::new(RwLock::new(Inner::new()))
        }
    }
}
