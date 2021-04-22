//! A basic server implementation for [Serial Studio](https://github.com/Serial-Studio/Serial-Studio)

use std::{
    io::Write,
    net::TcpListener,
    sync::mpsc::{self, Sender},
};

pub mod data;
pub mod friendly;

use data::TelemetryFrame;
use std::thread;

struct State {
    new_frame: Option<TelemetryFrame>,
    running: bool,
}

/// A single-connection server for SerialStudio
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

    /// Start the server
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

            loop {
                println!("Waiting for a SerialStudio session to attach");

                // Get a stream
                let stream = listener.accept();

                if stream.is_ok() {
                    let mut stream = stream.unwrap();
                    println!("Connection established!");

                    // Event loop
                    loop {
                        let new_data: State = rx.recv().unwrap();

                        // Kill on stop
                        if !new_data.running {
                            return;
                        }

                        // Send frame
                        if new_data.new_frame.is_some() {
                            // Get data
                            let obj = new_data.new_frame.unwrap();

                            // Serialize
                            let json = serde_json::to_string(&obj).unwrap();

                            // Send
                            let result = stream.0.write(format!("/*{}*/\n", json).as_bytes());

                            if result.is_err() {
                                println!("Failed to write telemetry update over TCP");
                                break;
                            }
                        }
                    }
                }

                println!("SerialStudio disconnected");
            }
        });
    }

    /// Stop the server
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

    /// Publish a new frame
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
