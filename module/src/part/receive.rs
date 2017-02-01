use ::types::Message;
use ::utils;

use std::sync::mpsc;

pub trait ReceiveMessage: Send {
    fn receive_message(&mut self) -> Message;
}

pub struct MessageReceiver {
    _drop_join: utils::DropJoin,
}

impl MessageReceiver {
    pub fn start<R>(mut receive_message: R) -> (Self, mpsc::Receiver<Message>)
        where R: ReceiveMessage + 'static {
        let (tx, rx) = mpsc::sync_channel(1024);
        let drop_join = utils::DropJoin::spawn_loop("message receiver", move |ref mut join_channel| {
            if let Err(error) = tx.send(receive_message.receive_message()) {
                info!("MessageReceiver failed to send: {}", error);
                if let Err(error) = join_channel.send(utils::Join) {
                    panic!(error);
                }
            }
        });
        (MessageReceiver { _drop_join: drop_join }, rx)
    }
}
