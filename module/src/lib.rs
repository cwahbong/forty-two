use ::types::Error;
use ::types::Result;
pub use ::types::Event;

pub mod functions;
pub mod types;
pub mod service;

// Need a sender for high level event communication
//
// Regist
// * Function
// * Stream Receiver
//
// Sender for
// * Remote function call
// * Stream Sender

// pub trait Module {
//     fn send_event(&mut self, event: Event) -> Result<()> {
//         // TODO
//         Ok(())
//     }
// 
//     fn handle_event(&mut self, event: Event) -> Result<()>;
// }
// 
// pub trait NewModule {
//     type Module: Module;
//     fn new_module(&self) -> Self::Module;
// }

use std::thread;
use std::sync::mpsc;

struct EventSender {
    inner_thread: Option<thread::JoinHandle<()>>,
    sender: Option<mpsc::SyncSender<Event>>,
}

impl EventSender {
    pub fn new() -> Self {
        EventSender {
            inner_thread: None,
            sender: None,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        if self.inner_thread.is_some() || self.sender.is_some() {
            return Err(Error::InvalidStatus);
        }

        let (tx, rx) = mpsc::sync_channel(1024);
        self.sender = Some(tx);
        self.inner_thread = Some(thread::spawn(move || {
            loop {
                let event = rx.recv();
                // TODO send event to underlying protocol
            }
        }));
        Ok(())
    }

    pub fn send(&mut self, event: Event) -> Result<()> {
        match self.sender.as_ref() {
            None => Err(Error::InvalidStatus),
            Some(sender) => {
                sender.send(event);
                Ok(())
            }
        }
    }

    pub fn end(&mut self) -> Result<()> {
        match self.inner_thread.take() {
            None => Err(Error::InvalidStatus),
            Some(inner_thread) => {
                inner_thread.join().expect("join event sender failed");
                self.sender = None;
                Ok(())
            }
        }
    }
}

struct EventReceiver {
    inner_thread: Option<thread::JoinHandle<()>>,
    receiver: Option<mpsc::Receiver<Event>>,
}

impl EventReceiver {
    pub fn new() -> Self {
        EventReceiver {
            inner_thread: None,
            receiver: None,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        if self.inner_thread.is_some() || self.receiver.is_some() {
            return Err(Error::InvalidStatus);
        }

        Ok(())
    }

    pub fn recv(&mut self)

    pub fn end(&mut self) -> Result<()> {
        match self.inner_thread.take() {
            None => Err(Error::InvalidStatus),
            Some(inner_thread) => {
                inner_thread.join().expect("join event receiver failed");
                self.receiver = None;
                Ok(())
            }
        }
    }
}

struct Module {
    event_sender: EventSender,
    event_receiver: EventReceiver,
}

impl Module {
    pub fn new() -> Self {
        Module {
            event_sender: EventSender::new(),
            event_receiver: EventReceiver::new(),
        }
    }

    pub fn run(&mut self) {
        self.event_sender.start();
        // prepare worker pool
        self.event_receiver.start();
    }
}

// pub fn main<N: NewModule>(new_module: N) {
//     // TODO make use of this module object
// }
