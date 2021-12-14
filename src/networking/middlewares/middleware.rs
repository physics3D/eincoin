use std::sync::{Arc, Mutex};

use bus::Bus;
use std::sync::mpsc::Sender;

use crate::{blockchain::Blockchain, networking::InternalMessage};

pub trait Middleware {
    fn on_message(
        &mut self,
        message: &InternalMessage,
        preprocessing_sender: &Sender<InternalMessage>,
        postprocessing_sender: Arc<Mutex<Bus<InternalMessage>>>,
        chain: &mut Blockchain,
    );
}
