use ::types::Uuid;

pub struct Arguments;
pub struct ReturnValue;

pub trait Call {
    fn name(&self) -> String;
    fn call(&mut self, id: &Uuid, arguments: Arguments) -> ReturnValue;
}

// pub trait Stream {
//     fn name() -> &str;
//     fn stream(id: &Uuid, arguments: Arguments) -> ???;
// }
