
use std::thread;
use std::sync::mpsc;

pub struct Join;

pub struct DropJoin {
    signal_on_drop: bool,
    join_channel: mpsc::Sender<Join>,
    join_handle: Option<thread::JoinHandle<()>>,
}

impl DropJoin {
    pub fn spawn_loop<F>(name: &str, mut loop_body: F) -> Self
        where F: FnMut(&mut mpsc::Sender<Join>) -> () + Send + 'static
    {
        let (tx, rx) = mpsc::channel();
        let mut join_channel = tx.clone();
        let join_handle = Some(thread::Builder::new()
            .name(String::from(name))
            .spawn(move || {
                let thread_current = thread::current();
                let thread_name = thread_current.name().unwrap();
                info!("Start thread '{}'", thread_name);
                loop {
                    match rx.try_recv() {
                        Err(mpsc::TryRecvError::Empty) => loop_body(&mut join_channel),
                        Err(mpsc::TryRecvError::Disconnected) => panic!(),
                        Ok(Join) => break,
                    }
                }
                info!("End thread '{}'", thread_name);
            })
            .unwrap());
        DropJoin {
            signal_on_drop: true,
            join_channel: tx,
            join_handle: join_handle,
        }
    }

    pub fn signal_on_drop(mut self, signal_on_drop: bool) -> Self {
        self.signal_on_drop = signal_on_drop;
        self
    }

    pub fn name(&mut self) -> &str {
        self.join_handle.as_ref().unwrap().thread().name().unwrap()
    }
}

impl Drop for DropJoin {
    fn drop(&mut self) {
        let name = String::from(self.name());
        if self.signal_on_drop {
            if self.join_channel.send(Join).is_err() {
                info!("Join channel for '{}' is closed.", name);
            } else {
                info!("Send signal to join channel for '{}'", name);
            }
        } else {
            info!("Do not send signal to join channel for '{}'", name);
        }
        if self.join_handle.take().unwrap().join().is_err() {
            error!("Join '{}' failed", name);
        }
    }
}
