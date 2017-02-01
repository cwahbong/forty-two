use ::api;
use ::part;
use ::part::EventDispatcher;
use ::part::MessageReceiver;
use ::part::MessageSender;

pub struct Module {
    // TODO maybe graceful shutdown, prevent errors such as "sending / receiving on a closed
    // channel"
    message_sender: Option<MessageSender>,
    message_receiver: Option<MessageReceiver>,
    event_dispatcher: Option<EventDispatcher>,
}

impl Module {
    pub fn new(message_sender: MessageSender, message_receiver: MessageReceiver, event_dispatcher: EventDispatcher) -> Self {
        Module {
            message_sender: Some(message_sender),
            message_receiver: Some(message_receiver),
            event_dispatcher: Some(event_dispatcher),
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        self.event_dispatcher.take();
        self.message_receiver.take();
        self.message_sender.take();
    }
}

pub fn start<R, S>(api_table: api::Table, receive_message: R, send_message: S) -> Module
    where R: part::receive::ReceiveMessage + 'static,
          S: part::send::SendMessage + 'static {
    let (message_sender, sender_channel) = MessageSender::start(send_message);
    let (message_receiver, receiver_channel) = MessageReceiver::start(receive_message);
    let event_dispatcher = EventDispatcher::start(api_table, receiver_channel, sender_channel);
    Module::new(message_sender, message_receiver, event_dispatcher)
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use ::api::Table;
    use ::types::{Arguments, Event, Message, Uuid};
    use ::part::send::SendMessage;
    use ::part::receive::ReceiveMessage;

    struct StartEndSendMessage;

    impl SendMessage for StartEndSendMessage {
        fn send_message(&mut self, _: Message) {}
    }

    struct StartEndReceiveMessage;

    impl ReceiveMessage for StartEndReceiveMessage {
        fn receive_message(&mut self) -> Message {
            Message {
                sender: Uuid::nil(),
                receiver: Uuid::nil(),
                event: Event {
                    name: String::from("end"),
                    kind: String::from("module"),
                    arguments: Arguments {},
                }
            }
        }
    }

    #[test]
    fn start_end() {
        env_logger::init().unwrap();
        super::start(Table::new(), StartEndReceiveMessage, StartEndSendMessage);
    }
}
