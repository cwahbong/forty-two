use std::collections;
use std::sync::Arc;

pub struct Arguments;
pub struct ReturnValue;

#[derive(Clone)]
pub struct Caller {
}

impl Caller {
    // XXX let it return future value
    pub fn call(&self, name: &str, arguments: Arguments) -> ReturnValue {
        // TODO regist "Once" receiver
        // TODO push event into sender
        // TODO wait response from invoker recv queue
        ReturnValue {}
    }
}

pub struct MetaCaller {
    caller_map: collections::HashMap<String, Caller>,
}

impl MetaCaller {
    pub fn to(&self, caller_name: &str) -> Option<Caller> {
        self.caller_map.get(caller_name).cloned()
    }
}

pub struct Callee {
}

pub trait Call {
    fn name(&self) -> String;
    fn call(&self, meta_caller: &MetaCaller, arguments: Arguments) -> ReturnValue;
}

pub struct Table {
    // notify_map: collections::HashMap<String, Arc<Box<Call>>,
    call_map: collections::HashMap<String, Arc<Box<Call + Send + Sync>>>,
}

impl Table {
    pub fn new() -> Self {
        Table { call_map: collections::HashMap::new() }
    }

    pub fn call(&self, name: &str) -> Option<Arc<Box<Call + Send + Sync>>> {
        self.call_map.get(name).cloned()
    }
}
