use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use bus::{Bus, BusReader};
use log::{error, info};
use std::sync::mpsc::Sender;

use crate::consts::{BUFFER_SIZE, NETWORKING_LOOP_SLEEP_TIME};

use super::{
    message::{MessageDest, MessageSource},
    InternalMessage,
};

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
    ) -> Self {
        Self {
            server: Some(TcpListener::bind(addr).unwrap()),
            incoming_queue_sender,
            outgoing_queue_receiver_adder,
        }
    }

    pub fn start_networking(&mut self) {
        let sender = self.incoming_queue_sender.clone();
        let receiver_adder = self.outgoing_queue_receiver_adder.clone();

        let server = self.server.take().unwrap();

        thread::spawn(move || loop {
            match server.accept() {
                Ok((stream, socketaddr)) => {
                    info!("New connection from {}", socketaddr);
                    handle_connection(
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

fn handle_connection(
    mut stream: TcpStream,
    sender: Sender<InternalMessage>,
    mut receiver: BusReader<InternalMessage>,
) {
    stream.set_nonblocking(true).unwrap();

    thread::spawn(move || {
        let address = stream.peer_addr().unwrap().to_string();

        loop {
            if let Ok(msg) = receiver.try_recv() {
                if msg.should_be_send_to(&address) {
                    stream
                        .write_all(&bincode::serialize(&msg.message).unwrap())
                        .unwrap();
                }
            }

            let mut buf = [0; BUFFER_SIZE];
            if let Ok(bytes) = stream.read(&mut buf) {
                // client shut down
                if bytes == 0 {
                    info!("The client at {} shut down", address);
                    break;
                }

                let message = InternalMessage::from_message(
                    bincode::deserialize(&buf[0..bytes]).unwrap(),
                    MessageSource::Foreign(address.clone()),
                    MessageDest::Localhost,
                );

                sender.send(message).unwrap();
            }

            // should help the cpu
            thread::sleep(Duration::from_millis(NETWORKING_LOOP_SLEEP_TIME));
        }
    });
}
