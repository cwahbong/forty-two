use fte_rwlock_module;
use std::sync::{Arc, RwLock};

use modules::Inner;
use result::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Args {
    pub name: String,
}

pub struct InnerCall;

impl fte_rwlock_module::InnerCall for InnerCall {
    type Inner = Inner;
    type Args = Args;
    type Rets = bool;
    type Error = Error;

    fn run(inner: &Arc<RwLock<Inner>>, args: Args) -> Result<bool> {
        let acquired = inner.read().unwrap();
        acquired.query(&args.name)
    }
}
