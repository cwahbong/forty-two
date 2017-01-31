use ::types::Event;
use ::utils;

use std::sync::mpsc;

pub trait ReceiveEvent: Send {
    fn receive_event(&mut self) -> Event;
}

pub struct EventReceiver {
    _drop_join: utils::DropJoin,
}

impl EventReceiver {
    pub fn start<R>(mut receive_event: R) -> (Self, mpsc::Receiver<Event>)
        where R: ReceiveEvent + 'static {
        let (tx, rx) = mpsc::sync_channel(1024);
        let drop_join = utils::DropJoin::spawn_loop("event receiver", move |ref mut join_channel| {
            if let Err(error) = tx.send(receive_event.receive_event()) {
                info!("EventReceiver failed to send: {}", error);
                if let Err(error) = join_channel.send(utils::Join) {
                    panic!(error);
                }
            }
        });
        (EventReceiver { _drop_join: drop_join }, rx)
    }
}
