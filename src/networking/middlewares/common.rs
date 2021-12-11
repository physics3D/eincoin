use log::warn;

use crate::blockchain::{Block, Blockchain};

pub fn verify_and_append_block_to_chain(chain: &mut Blockchain, block: &Block) {
    if chain.verify_new_block(&block) {
        chain.push_block(block.clone());
    } else {
        warn!("Received a wrong block");
    }
}
