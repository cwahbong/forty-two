use ::types::Error;
use ::types::Result;
pub use ::types::Event;
pub use ::types::EventArguments;

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

extern crate threadpool;

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

        let (tx, rx) = mpsc::sync_channel(1024);
        self.receiver = Some(rx);
        self.inner_thread = Some(thread::spawn(move || {
            // XXX receive event from underlying protocol
            let event = Event {
                name: String::from("dummy"),
                kind: String::from("dummy"),
                arguments: EventArguments {},
            };
            tx.send(event);
        }));
        Ok(())
    }

    pub fn recv(&mut self) -> Result<Event> {
        match self.receiver.as_ref() {
            None => Err(Error::InvalidStatus),
            Some(receiver) => Ok(try!(receiver.recv())),
        }
    }

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

struct EventWorker {
    inner_pool: Option<threadpool::ThreadPool>,
}

impl EventWorker {
    // TODO set service info
    // TODO set sender
    pub fn new() -> Self {
        EventWorker {
            inner_pool: None,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        if self.inner_pool.is_some() {
            return Err(Error::InvalidStatus);
        }

        self.inner_pool = Some(threadpool::ThreadPool::new(2));
        Ok(())
    }

    // TODO handle from service info
    pub fn handle(&mut self, event: Event) {
        // TODO use inner pool to run the api
    }

    pub fn end(&mut self) -> Result<()> {
        if self.inner_pool.take().is_none() {
            return Err(Error::InvalidStatus);
        }
        // TODO maybe waiting all api to end
        // TODO other cleanups
        Ok(())
    }
}

struct Module {
    event_sender: EventSender,
    event_worker: EventWorker,
    event_receiver: EventReceiver,
}

impl Module {
    // set service info (api)
    pub fn new() -> Self {
        Module {
            event_sender: EventSender::new(),
            event_worker: EventWorker::new(),
            event_receiver: EventReceiver::new(),
        }
    }

    fn start(&mut self) {
        self.event_sender.start();
        self.event_worker.start();
        self.event_receiver.start();
    }

    fn end(&mut self) {
        self.event_receiver.end();
        self.event_worker.end();
        self.event_sender.end();
    }

    pub fn run(&mut self) {
        self.start();
        loop {
            match self.event_receiver.recv() {
                Err(error) => {
                    self.end();
                    panic!(error);
                }
                Ok(event) => {
                    if event.kind == ".module" {
                        if event.name == "end" {
                            self.end();
                        }
                    } else {
                        self.event_worker.handle(event);
                    }
                }
            }
        }
    }
}

// pub fn main<N: NewModule>(new_module: N) {
//     // TODO make use of this module object
// }
