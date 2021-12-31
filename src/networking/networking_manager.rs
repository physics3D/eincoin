use std::sync::{Arc, Mutex};

use bus::Bus;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::{blockchain::Blockchain, consts::BUFFER_SIZE, util::LogExpect};

use super::{middlewares::Middleware, Client, InternalMessage, Server};

pub struct NetworkingManager {
    client: Option<Client>,
    server: Option<Server>,
    incoming_queue_sender: Sender<InternalMessage>,
    incoming_queue_receiver: Option<Receiver<InternalMessage>>,
    outgoing_queue_sender: Arc<Mutex<Bus<InternalMessage>>>,
    middlewares: Vec<Box<dyn Middleware>>,
}

impl NetworkingManager {
    pub fn new(addr: Option<String>, server_port: Option<String>) -> Self {
        let (incoming_queue_sender, incoming_queue_receiver) = channel();
        let outgoing_queue_sender = Arc::new(Mutex::new(Bus::new(BUFFER_SIZE)));

        let mut local_server = None;
        let mut local_client = None;

        if let Some(port) = server_port {
            local_server = Some(
                Server::new(
                    "127.0.0.1:".to_string() + &port,
                    incoming_queue_sender.clone(),
                    outgoing_queue_sender.clone(),
                )
                .log_expect(&format!(
                    "The port at {} is already in use. Please use another port",
                    port
                )),
            );
        }

        if addr.is_some() {
            let client_addr = addr.unwrap();
            local_client = Some(
                Client::new(
                    client_addr.clone(),
                    incoming_queue_sender.clone(),
                    outgoing_queue_sender.lock().unwrap().add_rx(),
                )
                .log_expect(&format!("Server at {} unavailable", client_addr)),
            );
        }

        let networking_manager = Self {
            client: local_client,
            server: local_server,
            incoming_queue_sender,
            incoming_queue_receiver: Some(incoming_queue_receiver),
            outgoing_queue_sender,
            middlewares: vec![],
        };

        networking_manager
    }

    pub fn add_middleware(&mut self, middleware: impl Middleware + 'static) {
        self.middlewares.push(Box::new(middleware));
    }

    fn start_networking_event_loop(&mut self, chain: &mut Blockchain) {
        loop {
            let message = self
                .incoming_queue_receiver
                .as_ref()
                .unwrap()
                .recv()
                .unwrap();
            self.run_middlewares(message, chain);
        }
    }

    pub fn run_middlewares(&mut self, message: InternalMessage, chain: &mut Blockchain) {
        for middleware in &mut self.middlewares {
            middleware.on_message(
                &message,
                &self.incoming_queue_sender,
                self.outgoing_queue_sender.clone(),
                chain,
            );
        }
    }

    pub fn start_networking(&mut self, chain: &mut Blockchain) {
        self.start_client_server();
        self.start_networking_event_loop(chain);
    }

    pub fn start_client_server(&mut self) {
        if let Some(client) = &mut self.client {
            client.start_networking();
        }
        if let Some(server) = &mut self.server {
            server.start_networking();
        }
    }

    pub fn get_sender(&self) -> Arc<Mutex<Bus<InternalMessage>>> {
        self.outgoing_queue_sender.clone()
    }

    pub fn get_receiver(&mut self) -> Option<Receiver<InternalMessage>> {
        self.incoming_queue_receiver.take()
    }
}
