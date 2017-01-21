#[macro_use]
extern crate log;
extern crate threadpool;

use std::sync::mpsc;

pub use ::types::Event;
pub use ::types::EventArguments;

pub mod api;
pub mod part;
pub mod types;
pub mod utils;

// Need a sender for high level event communication
//
// Regist
// * Function
// * Stream Receiver
//
// Sender for
// * Remote function call
// * Stream Sender

struct EventSender {
    _drop_join: utils::DropJoin,
}

impl EventSender {
    pub fn start<S>(mut send_event: S) -> (Self, mpsc::SyncSender<Event>)
        where S: part::SendEvent + 'static {
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

struct EventReceiver {
    _drop_join: utils::DropJoin,
}

impl EventReceiver {
    pub fn start<R>(mut receive_event: R) -> (Self, mpsc::Receiver<Event>)
        where R: part::ReceiveEvent + 'static {
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

struct EventHandler {
    api_table: api::Table,
    thread_pool: threadpool::ThreadPool,
}

impl EventHandler {
    pub fn start(api_table: api::Table) -> Self {
        EventHandler {
            api_table: api_table,
            thread_pool: threadpool::ThreadPool::new(2),
        }
    }

    fn handle_module(&mut self, join_channel: &mut mpsc::Sender<utils::Join>, event: Event) {
        if event.name == "end" {
            info!("Module end event");
            if let Err(error) = join_channel.send(utils::Join) {
                panic!(error);
            }
        }
    }

    fn handle_caller(&mut self, _event: Event) {
        // TODO handle response of my calls
    }

    fn handle_callee(&mut self, _sender_channel: &mut mpsc::SyncSender<Event>, _event: Event) {
        // TODO handle request and notififications to me
    }

    pub fn handle(&mut self,
                  join_channel: &mut mpsc::Sender<utils::Join>,
                  sender_channel: &mut mpsc::SyncSender<Event>,
                  event: Event) {
        match event.kind.as_str() {
            "module" => self.handle_module(join_channel, event),
            "response" => self.handle_caller(event),
            "request" | "notify" => self.handle_callee(sender_channel, event),
            _ => warn!("Unknown event kind {}", event.kind),
        }
    }
}

struct EventDispatcher {
    _drop_join: utils::DropJoin,
}

impl EventDispatcher {
    pub fn start(api_table: api::Table,
                 receiver_channel: mpsc::Receiver<Event>,
                 mut sender_channel: mpsc::SyncSender<Event>)
                 -> Self {
        let mut event_handler = EventHandler::start(api_table);
        let drop_join = utils::DropJoin::spawn_loop("event dispatcher", move |ref mut join_channel| {
            match receiver_channel.recv() {
                Err(error) => {
                    error!("Receiver channel recv failed: {}", error);
                    // TODO maybe some cleanups
                    if let Err(error) = join_channel.send(utils::Join) {
                        panic!(error);
                    }
                }
                Ok(event) => {
                    event_handler.handle(join_channel, &mut sender_channel, event);
                }
            }
        }).signal_on_drop(false);
        // DO NOT send message to inner thread, module should be ended by outer signal

        // TODO maybe waiting all api to end on drop
        // TODO maybe other cleanups on drop
        EventDispatcher { _drop_join: drop_join }
    }
}

pub struct Module {
    // TODO maybe graceful shutdown, prevent errors such as "sending / receiving on a closed
    // channel"
    event_sender: Option<EventSender>,
    event_receiver: Option<EventReceiver>,
    event_dispatcher: Option<EventDispatcher>,
}

impl Module {
    pub fn start<R, S>(api_table: api::Table, receive_event: R, send_event: S) -> Self
        where R: part::ReceiveEvent + 'static,
              S: part::SendEvent + 'static {
        let (event_sender, sender_channel) = EventSender::start(send_event);
        let (event_receiver, receiver_channel) = EventReceiver::start(receive_event);
        let event_dispatcher = EventDispatcher::start(api_table, receiver_channel, sender_channel);
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
