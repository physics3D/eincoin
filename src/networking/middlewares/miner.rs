use std::{
    sync::mpsc::{channel, Sender},
    thread,
};

use log::info;

use crate::{
    blockchain::Block,
    networking::{
        message::{MessageDest, MessageSource},
        InternalMessage, MessageType,
    },
};

pub struct Miner {
    killswitch_sender: Option<Sender<()>>,
}

impl Miner {
    pub fn new() -> Self {
        Self {
            killswitch_sender: None,
        }
    }

    pub fn mine(&mut self, mut block: Block, result_sender: Sender<InternalMessage>) {
        info!("Started mining");
        let (killswitch_sender, killswitch_receiver) = channel();
        self.killswitch_sender = Some(killswitch_sender);

        thread::spawn(move || loop {
            if killswitch_receiver.try_recv().is_ok() {
                break;
            }

            if block.verify_nonce() {
                info!("Solved a block: {}", block.nonce);
                result_sender
                    .send(InternalMessage::new(
                        MessageType::MinedBlock(block),
                        MessageSource::Localhost,
                        MessageDest::Localhost,
                    ))
                    .unwrap();
                break;
            } else {
                block.nonce += 1;
            }
        });
    }

    pub fn abort(&mut self) {
        if let Some(sender) = &self.killswitch_sender {
            match sender.send(()) {
                Ok(_) => info!("Killing miner"),
                Err(_) => {}
            };

            self.killswitch_sender = None;
        }
    }
}
