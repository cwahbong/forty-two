use ::types::Message;
use ::utils;

use std::sync::mpsc;

pub trait SendMessage: Send {
    fn send_message(&mut self, message: Message);
}

pub struct MessageSender {
    _drop_join: utils::DropJoin,
}

impl MessageSender {
    pub fn start<S>(mut send_message: S) -> (Self, mpsc::SyncSender<Message>)
        where S: SendMessage + 'static {
        let (tx, rx) = mpsc::sync_channel(1024);
        let drop_join = utils::DropJoin::spawn_loop("message sender", move |ref mut join_channel| {
            match rx.recv() {
                Err(error) => {
                    info!("MessageSender failed to recv: {}", error);
                    if let Err(error) = join_channel.send(utils::Join) {
                        panic!(error);
                    }
                }
                Ok(event) => {
                    send_message.send_message(event);
                }
            }
        });
        (MessageSender { _drop_join: drop_join }, tx)
    }
}
