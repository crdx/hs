#![warn(future_incompatible)]
#![warn(missing_copy_implementations)]
#![warn(nonstandard_style)]
#![warn(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::option_if_let_else)]

use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::exit;

use docopt::Docopt;
use serde::Deserialize;
use unindent::unindent;

mod command;
mod history;
mod util;

fn get_program_name() -> Option<String> {
    env::current_exe()
        .ok()?
        .file_name()?
        .to_os_string()
        .into_string()
        .ok()
}

fn usage() -> String {
    let usage = format!(
        "
        Usage:
            {} [options] [ --file <path> ]

        Parse timestamped Bash history and output it sorted.

        Options:
            -f, --file <path>    Timestamped Bash history file
            -h, --help           Show help
    ",
        get_program_name().unwrap()
    );

    unindent(&usage)
}

#[derive(Debug, Deserialize)]
struct Opts {
    flag_file: Option<String>,
    flag_help: bool,
}

fn get_path(opts: &Opts) -> PathBuf {
    if let Some(path) = &opts.flag_file {
        PathBuf::from(path)
    } else {
        let mut path = PathBuf::from(env::var_os("HOME").unwrap());
        path.push(".bash_history");
        path
    }
}

fn main() {
    let opts: Opts = Docopt::new(usage())
        .and_then(|a| a.deserialize())
        .unwrap_or_else(|a| a.exit());

    let path = get_path(&opts);
    let lines = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    let mut history = history::History::default();
    history.add(&lines);

    match history.display() {
        Err(e) if e.kind() != ErrorKind::BrokenPipe => {
            eprintln!("Error: {}", e);
            exit(1);
        }
        _ => {
            exit(0);
        }
    }
}
