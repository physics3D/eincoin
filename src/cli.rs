use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, SharedLogger, TermLogger, TerminalMode, WriteLogger};
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

use crate::consts::LOG_CONFIG;

/// A shitty try at implementing a cryptocurrency
#[derive(StructOpt, Clone)]
pub struct CliArgs {
    /// the log level
    #[structopt(short, long, default_value = "Info", env = "RUST_LOG")]
    log_level: LevelFilter,
    /// save the log to this file
    #[structopt(short = "f", long, parse(from_os_str))]
    log_file: Option<PathBuf>,
    #[structopt(subcommand)]
    pub subcommand: Command,
}

#[derive(StructOpt, Clone)]
pub enum Command {
    /// Generate a Rsa keypair for your wallet
    GenKey {
        /// Pass a file if you want to save the keypair in a file. Otherwise, it will print to stdout
        file: Option<String>,
    },
    /// Generate your public key from your private key
    GenPubKey {
        /// The file with your wallet's private key
        #[structopt(parse(from_os_str))]
        private_key_file: PathBuf,
    },
    /// Generate completions for your shell
    GenCompletions {
        /// Your shell
        shell: String,
        /// The file to write the completion script to
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
    /// Connect a full node to the eincoin network
    FullNode {
        /// The address of the eincoin server to connect to
        addr: String,
        /// The port of the server
        #[structopt(short, long, default_value = "3333")]
        port: String,
        /// Attempt to mine new blocks
        #[structopt(short, long, requires("private-key-file"))]
        miner: bool,
        /// The file with your wallet's private key
        #[structopt(short = "k", long = "key-file", parse(from_os_str))]
        private_key_file: Option<PathBuf>,
        /// The port to open a server on
        #[structopt(short, long)]
        server: Option<String>,
    },
    /// Start a server node which creates a new blockchain
    Genesis {
        /// The port of the server
        #[structopt(short, long, default_value = "3333")]
        server: String,
        /// The file with your wallet's private key
        #[structopt(parse(from_os_str))]
        private_key_file: PathBuf,
    },
    /// Init a transaction on the eincoin network
    Transaction {
        /// The address of the eincoin server to connect to
        addr: String,
        /// The port of the server
        #[structopt(short, long, default_value = "3333")]
        port: String,
        /// The amount of Eincoin to send
        amount: u32,
        /// The file with the payee's public key
        #[structopt(parse(from_os_str))]
        payee_public_key: PathBuf,
        /// The file with your wallet's private key
        #[structopt(parse(from_os_str))]
        private_key_file: PathBuf,
    },
    /// View your wallet's balance
    Balance {
        /// The address of the eincoin server to connect to
        addr: String,
        /// The port of the server
        #[structopt(short, long, default_value = "3333")]
        port: String,
        /// The file with your wallet's private key
        #[structopt(parse(from_os_str))]
        private_key_file: PathBuf,
    },
    /// execute several commands interactively
    Interactive {
        /// The address of the eincoin server to connect to
        addr: String,
        /// The port of the server
        #[structopt(short, long, default_value = "3333")]
        port: String,
        /// The file with your wallet's private key
        #[structopt(parse(from_os_str))]
        private_key_file: PathBuf,
    },
}

pub fn setup_loggers(cli_args: &CliArgs) {
    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        cli_args.log_level,
        LOG_CONFIG.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];

    if let Some(path) = &cli_args.log_file {
        loggers.push(WriteLogger::new(
            cli_args.log_level,
            LOG_CONFIG.clone(),
            File::create(path).unwrap(),
        ));
    }

    CombinedLogger::init(loggers).unwrap();
}
