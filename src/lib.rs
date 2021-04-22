use std::{
    net::{TcpListener, ToSocketAddrs},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
};

pub mod data;

use data::TelemetryFrame;
use std::thread;


struct State {
    new_frame: Option<TelemetryFrame>,
    running: bool,
}

pub struct SerialStudioSource {
    running: bool,
    chan_to_thread: Option<Sender<State>>,
}

impl SerialStudioSource {
    pub fn new() -> Self {
        Self {
            running: false,
            chan_to_thread: None,
        }
    }

    pub fn start(&mut self, bind_addr: String) {
        // Build a thread-safe channel for sending data
        let (tx, rx) = mpsc::channel();
        self.chan_to_thread = Some(tx);

        // Send initial state to the thread
        let _ = self
            .chan_to_thread
            .as_ref()
            .unwrap()
            .send(State {
                new_frame: None,
                running: true,
            })
            .unwrap();
        self.running = true;

        // Execution thread
        thread::spawn(move || {
            // Create a listener
            let listener = TcpListener::bind(bind_addr).unwrap();

            for stream in listener.incoming() {
                let stream = stream.unwrap();

                let new_data: State = rx.recv().unwrap();

                println!("Connection established!");
            }
        });
    }

    pub fn stop(&mut self) {
        self.running = false;
        let _ = self
            .chan_to_thread
            .as_ref()
            .unwrap()
            .send(State {
                new_frame: None,
                running: false,
            })
            .unwrap();
    }

    pub fn publish(&mut self, frame: TelemetryFrame) {
        if self.running && self.chan_to_thread.is_some() {
            let _ = self
                .chan_to_thread
                .as_ref()
                .unwrap()
                .send(State {
                    new_frame: Some(frame),
                    running: true,
                })
                .unwrap();
        }
    }
}
