use ::types::Event;

pub trait SendEvent: Send {
    fn send_event(&mut self, event: Event);
}

pub trait ReceiveEvent: Send {
    fn receive_event(&mut self) -> Event;
}
