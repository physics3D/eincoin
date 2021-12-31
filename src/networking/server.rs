use std::{
    io::Result,
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

use bus::Bus;
use log::{error, info};
use std::sync::mpsc::Sender;

use crate::networking::handle_stream;

use super::InternalMessage;

pub struct Server {
    server: Option<TcpListener>,
    incoming_queue_sender: Sender<InternalMessage>,
    outgoing_queue_receiver_adder: Arc<Mutex<Bus<InternalMessage>>>,
}

impl Server {
    pub fn new(
        addr: String,
        incoming_queue_sender: Sender<InternalMessage>,
        outgoing_queue_receiver_adder: Arc<Mutex<Bus<InternalMessage>>>,
    ) -> Result<Self> {
        Ok(Self {
            server: Some(TcpListener::bind(addr)?),
            incoming_queue_sender,
            outgoing_queue_receiver_adder,
        })
    }

    pub fn start_networking(&mut self) {
        let sender = self.incoming_queue_sender.clone();
        let receiver_adder = self.outgoing_queue_receiver_adder.clone();

        let server = self.server.take().unwrap();

        thread::spawn(move || loop {
            match server.accept() {
                Ok((stream, socketaddr)) => {
                    info!("New connection from {}", socketaddr);
                    handle_stream(
                        stream,
                        sender.clone(),
                        receiver_adder.lock().unwrap().add_rx(),
                    );
                }
                Err(err) => error!("Couldn't connect to client because of {}", err),
            }
        });
    }
}
