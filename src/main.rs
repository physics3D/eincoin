use log::info;
use structopt::StructOpt;

use crate::cli::{setup_loggers, CliArgs, Command};
use crate::commands::{
    balance, full_node, gen_completions, gen_key, gen_pub_key, genesis, interactive, transaction,
};

mod blockchain;
mod cli;
mod commands;
mod consts;
mod networking;
mod util;

fn main() {
    let cli_args = CliArgs::from_args();

    setup_loggers(&cli_args);

    info!("Started eincoin node");

    match cli_args.subcommand {
        Command::GenKey { file } => {
            gen_key(file);
        }
        Command::GenPubKey { private_key_file } => {
            gen_pub_key(private_key_file);
        }
        Command::GenCompletions { shell, file } => {
            gen_completions(shell, file);
        }
        Command::FullNode {
            addr,
            port,
            miner,
            server,
            private_key_file,
        } => {
            full_node(addr, port, miner, server, private_key_file);
        }
        Command::Genesis {
            server,
            private_key_file,
        } => {
            genesis(server, private_key_file);
        }
        Command::Transaction {
            addr,
            port,
            amount,
            payee_public_key,
            private_key_file,
        } => {
            transaction(addr, port, amount, payee_public_key, private_key_file);
        }
        Command::Balance {
            addr,
            port,
            private_key_file,
        } => {
            balance(addr, port, private_key_file);
        }
        Command::Interactive {
            addr,
            port,
            private_key_file,
        } => {
            interactive(addr, port, private_key_file);
        }
    }

    info!("Terminating eincoin node");
}
