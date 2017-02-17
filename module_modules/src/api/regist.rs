use fte_rwlock_module;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use modules::Inner;
use result::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Args {
    pub name: String,
    pub addr: String,
    pub port: u16,
}

pub struct InnerCall;

impl fte_rwlock_module::InnerCall for InnerCall {
    type Inner = Inner;
    type Args = Args;
    type Rets = ();
    type Error = Error;

    fn run(inner: &Arc<RwLock<Inner>>, args: Args) -> Result<()> {
        let mut acquired = inner.write().unwrap();
        acquired.regist(args.name, SocketAddr::new(args.addr.parse()?, args.port))
    }
}
