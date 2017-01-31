use ::types::Event;
use ::utils;

use std::sync::mpsc;

pub trait SendEvent: Send {
    fn send_event(&mut self, event: Event);
}

pub struct EventSender {
    _drop_join: utils::DropJoin,
}

impl EventSender {
    pub fn start<S>(mut send_event: S) -> (Self, mpsc::SyncSender<Event>)
        where S: SendEvent + 'static {
        let (tx, rx) = mpsc::sync_channel(1024);
        let drop_join = utils::DropJoin::spawn_loop("event sender", move |ref mut join_channel| {
            match rx.recv() {
                Err(error) => {
                    info!("EventSender failed to recv: {}", error);
                    if let Err(error) = join_channel.send(utils::Join) {
                        panic!(error);
                    }
                }
                Ok(event) => {
                    send_event.send_event(event);
                }
            }
        });
        (EventSender { _drop_join: drop_join }, tx)
    }
}
