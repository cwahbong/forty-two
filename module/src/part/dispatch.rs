extern crate threadpool;

use ::api;
use ::types::Event;
use ::utils;

use std::sync::mpsc;

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

pub struct EventDispatcher {
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

