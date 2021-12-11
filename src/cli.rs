use log::{info, LevelFilter};
use simplelog::{
    ColorChoice, CombinedLogger, Config, SharedLogger, TermLogger, TerminalMode, WriteLogger,
};
use std::{
    fs::{write, File},
    path::PathBuf,
};
use structopt::StructOpt;

use crate::blockchain::Wallet;

/// A shitty try at implementing a cryptocurrency
#[derive(StructOpt, Clone)]
pub struct CliArgs {
    /// the log level
    #[structopt(short, long, default_value = "Info")]
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
        #[structopt(parse(from_os_str))]
        file: Option<PathBuf>,
    },
    /// Connect a full node to the eincoin network
    FullNode {
        /// The address of the eincoin server to connect to
        addr: String,
        /// The port of the server
        #[structopt(short, long, default_value = "3333")]
        port: String,
        /// Attempt to mine new blocks
        #[structopt(short, long)]
        miner: bool,
        /// The port to open a server on
        #[structopt(short, long)]
        server: Option<String>,
    },
    /// Start a server node which creates a new blockchain
    Genesis {
        /// The port of the server
        #[structopt(short, long, default_value = "3333")]
        port: String,
        /// The file with your wallet's private key
        #[structopt(parse(from_os_str))]
        private_key_file: PathBuf,
        /// The file with your wallet's public key
        #[structopt(parse(from_os_str))]
        public_key_file: PathBuf,
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
        /// The payee's public key
        #[structopt(parse(from_os_str))]
        payee_public_key: PathBuf,
        /// The file with your wallet's private key
        #[structopt(parse(from_os_str))]
        private_key_file: PathBuf,
        /// The file with your wallet's public key
        #[structopt(parse(from_os_str))]
        public_key_file: PathBuf,
    },
    /// View your wallets balance
    Balance {
        /// The address of the eincoin server to connect to
        addr: String,
        /// The port of the server
        #[structopt(short, long, default_value = "3333")]
        port: String,
        /// The file with your wallet's private key
        #[structopt(parse(from_os_str))]
        private_key_file: PathBuf,
        /// The file with your wallet's public key
        #[structopt(parse(from_os_str))]
        public_key_file: PathBuf,
    },
}

pub fn setup_loggers(cli_args: CliArgs) {
    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        cli_args.log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];

    if let Some(path) = cli_args.log_file {
        loggers.push(WriteLogger::new(
            cli_args.log_level,
            Config::default(),
            File::create(path).unwrap(),
        ));
    }

    CombinedLogger::init(loggers).unwrap();
}

pub fn gen_key(file: Option<PathBuf>) {
    info!("Generating keypair");
    let wallet = Wallet::new_random();
    let (private_key_string, public_key_string) = wallet.to_string();

    if let Some(path) = file {
        info!("Writing keypair to file {}", path.to_str().unwrap());
        write(
            PathBuf::from(path.to_str().unwrap().to_string() + ".priv"),
            private_key_string,
        )
        .unwrap();
        write(
            PathBuf::from(path.to_str().unwrap().to_string() + ".pub"),
            public_key_string,
        )
        .unwrap();
    } else {
        info!("Printing keypair to stdout");
        println!("{}", private_key_string);
        println!();
        println!("{}", public_key_string);
    }
}
