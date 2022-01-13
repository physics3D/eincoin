use lazy_static::lazy_static;
use log::LevelFilter;
use simplelog::{Config, ConfigBuilder, LevelPadding};

pub const DIFFICULTY: u32 = 2;
pub const INITIAL_COIN_AMOUNT: u32 = 100;
pub const MINING_REWARD: u32 = 1;
pub const KEY_PAIR_LENGTH: usize = 2048;

lazy_static! {
    // lazily create the needed start for a block hash: a null byte DIFFICULTY times
    pub static ref NEEDED_HASH_START: Vec<u8> = vec![0; DIFFICULTY as usize];

    pub static ref LOG_CONFIG: Config = ConfigBuilder::new()
    .set_target_level(LevelFilter::Off)
    .set_level_padding(LevelPadding::Right)
    .build();
}

pub const BUFFER_SIZE: usize = 4096;
