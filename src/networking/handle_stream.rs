use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::mpsc::Sender,
    thread,
};

use bus::BusReader;
use log::info;

use crate::consts::BUFFER_SIZE;

use super::{InternalMessage, MessageDest, MessageSource};

/// forward all messages
/// - from stream to sender
/// - from receiver to stream
pub fn handle_stream(
    mut stream: TcpStream,
    sender: Sender<InternalMessage>,
    mut receiver: BusReader<InternalMessage>,
) {
    stream.set_nonblocking(false).unwrap();

    let address = stream.peer_addr().unwrap().to_string();

    let mut stream_clone = stream.try_clone().unwrap();
    let address_clone = address.clone();

    // sender thread
    thread::spawn(move || loop {
        if let Ok(msg) = receiver.recv() {
            if msg.should_be_send_to(&address) {
                if let Err(_) = stream.write_all(&bincode::serialize(&msg.message).unwrap()) {
                    // connection shut down
                    info!("The connection to {} was shut down", address);
                    break;
                }
            }
        }
    });

    // receiver thread
    thread::spawn(move || {
        let mut buf = [0; BUFFER_SIZE];

        loop {
            if let Ok(bytes) = stream_clone.read(&mut buf) {
                if bytes == 0 {
                    // connection shut down
                    info!("The connection to {} was shut down", address_clone);
                    break;
                }

                let message = InternalMessage::from_message(
                    bincode::deserialize(&buf[0..bytes]).unwrap(),
                    MessageSource::Foreign(address_clone.clone()),
                    MessageDest::Localhost,
                );

                sender.send(message).unwrap();
            }
        }
    });
}
