use std::{
    io::{Read, Result, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

use super::{
    message::{MessageDest, MessageSource},
    InternalMessage, Message, MessageType,
};
use crate::consts::{BUFFER_SIZE, NETWORKING_LOOP_SLEEP_TIME};

use bus::BusReader;
use log::warn;
use std::sync::mpsc::Sender;

pub struct Client {
    client: Option<TcpStream>,
    incoming_queue_sender: Sender<InternalMessage>,
    outgoing_queue_receiver: Option<BusReader<InternalMessage>>,
}

impl Client {
    pub fn new(
        addr: String,
        incoming_queue_sender: Sender<InternalMessage>,
        outgoing_queue_receiver: BusReader<InternalMessage>,
    ) -> Result<Self> {
        Ok(Self {
            client: Some(TcpStream::connect(addr)?),
            incoming_queue_sender,
            outgoing_queue_receiver: Some(outgoing_queue_receiver),
        })
    }

    pub fn start_networking(&mut self) {
        let sender = self.incoming_queue_sender.clone();
        let mut receiver = self.outgoing_queue_receiver.take().unwrap();

        let mut stream = self.client.take().unwrap();
        stream.set_nonblocking(true).unwrap();

        stream
            .write_all(&bincode::serialize(&Message::new(MessageType::Connect)).unwrap())
            .unwrap();

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
                    // server shut down
                    if bytes == 0 {
                        warn!("The server at {} shut down", address);
                        break;
                    }

                    let message = InternalMessage::from_message(
                        bincode::deserialize(&buf[0..bytes]).unwrap(),
                        MessageSource::Foreign(address.clone()),
                        MessageDest::Localhost,
                    );

                    sender.send(message).unwrap();

                    // should help the cpu
                    thread::sleep(Duration::from_millis(NETWORKING_LOOP_SLEEP_TIME));
                }
            }
        });
    }
}
