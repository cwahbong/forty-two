use ::api;
use ::part;
use ::part::EventDispatcher;
use ::part::EventReceiver;
use ::part::EventSender;

pub struct Module {
    // TODO maybe graceful shutdown, prevent errors such as "sending / receiving on a closed
    // channel"
    event_sender: Option<EventSender>,
    event_receiver: Option<EventReceiver>,
    event_dispatcher: Option<EventDispatcher>,
}

impl Module {
    pub fn new(event_sender: EventSender, event_receiver: EventReceiver, event_dispatcher: EventDispatcher) -> Self {
        Module {
            event_sender: Some(event_sender),
            event_receiver: Some(event_receiver),
            event_dispatcher: Some(event_dispatcher),
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        self.event_dispatcher.take();
        self.event_receiver.take();
        self.event_sender.take();
    }
}

pub fn start<R, S>(api_table: api::Table, receive_event: R, send_event: S) -> Module
    where R: part::receive::ReceiveEvent + 'static,
          S: part::send::SendEvent + 'static {
    let (event_sender, sender_channel) = EventSender::start(send_event);
    let (event_receiver, receiver_channel) = EventReceiver::start(receive_event);
    let event_dispatcher = EventDispatcher::start(api_table, receiver_channel, sender_channel);
    Module::new(event_sender, event_receiver, event_dispatcher)
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use ::api::Table;
    use ::Event;
    use ::EventArguments;
    use ::part::send::SendEvent;
    use ::part::receive::ReceiveEvent;

    struct StartEndSendEvent;

    impl SendEvent for StartEndSendEvent {
        fn send_event(&mut self, _: Event) {}
    }

    struct StartEndReceiveEvent;

    impl ReceiveEvent for StartEndReceiveEvent {
        fn receive_event(&mut self) -> Event {
         Event {
                name: String::from("end"),
                kind: String::from("module"),
                arguments: EventArguments {}
            }
        }
    }

    #[test]
    fn start_end() {
        env_logger::init().unwrap();
        super::start(Table::new(), StartEndReceiveEvent, StartEndSendEvent);
    }
}
