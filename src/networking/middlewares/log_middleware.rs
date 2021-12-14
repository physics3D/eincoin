use std::sync::{Arc, Mutex};

use bus::Bus;
use log::info;
use std::sync::mpsc::Sender;

use crate::{blockchain::Blockchain, networking::InternalMessage};

use super::Middleware;

pub struct LogMiddleware;

impl Middleware for LogMiddleware {
    fn on_message(
        &mut self,
        message: &InternalMessage,
        _preprocessing_sender: &Sender<InternalMessage>,
        _postprocessing_sender: Arc<Mutex<Bus<InternalMessage>>>,
        _chain: &mut Blockchain,
    ) {
        info!(
            "Received a {} message from {} to {}",
            message.message.message_type.to_string(),
            message.source.to_string(),
            message.dest.to_string()
        );
    }
}
