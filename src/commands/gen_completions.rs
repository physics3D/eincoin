use std::{fs::File, path::PathBuf, process::exit, str::FromStr};

use log::error;
use structopt::{clap::Shell, StructOpt};

use crate::cli::CliArgs;

pub fn gen_completions(shell: String, file: PathBuf) {
    let shell_shell = Shell::from_str(&shell);
    if let Ok(shell_type) = shell_shell {
        CliArgs::clap().gen_completions_to(
            env!("CARGO_BIN_NAME"),
            shell_type,
            &mut match File::create(file) {
                Ok(the_file) => the_file,
                Err(err) => {
                    error!("Writing to file failed: {}", err);
                    exit(1);
                }
            },
        );
    } else {
        error!("Unknown shell!");
        exit(1);
    }
}
