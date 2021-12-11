use lazy_static::lazy_static;

pub const DIFFICULTY: u32 = 1;
pub const KEY_PAIR_LENGTH: usize = 2048;

// lazily create the needed start for a block hash: a null byte DIFFICULTY times
lazy_static! {
    pub static ref NEEDED_HASH_START: Vec<u8> = {
        let mut vec = vec![];
        for _ in 0..DIFFICULTY {
            vec.push(0u8);
        }
        vec
    };
}

pub const BUFFER_SIZE: usize = 4096;

pub const NETWORKING_LOOP_SLEEP_TIME: u64 = 100;
