use std::{
    io::{Result, Write},
    net::TcpStream,
};

use super::{handle_stream, InternalMessage, Message, MessageType};

use bus::BusReader;
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
        let receiver = self.outgoing_queue_receiver.take().unwrap();

        let mut stream = self.client.take().unwrap();
        stream.set_nonblocking(true).unwrap();

        stream
            .write_all(&bincode::serialize(&Message::new(MessageType::Connect)).unwrap())
            .unwrap();

        handle_stream(stream, sender, receiver);
    }
}
