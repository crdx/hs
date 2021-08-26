#![warn(future_incompatible)]
#![warn(missing_copy_implementations)]
#![warn(nonstandard_style)]
#![warn(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::option_if_let_else)]

use std::env;
use std::fs;
use std::io::{self, ErrorKind};
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
            {0} [options] [ --file PATH ]

        Parse timestamped Bash history and output it sorted.

        Options:
            -f, --file PATH    Timestamped Bash history file
            -h, --help         Show help
        ",
        get_program_name().unwrap()
    );

    unindent(&usage.trim())
}

fn parse_opts() -> Opts {
    if let Ok(opts) = Docopt::new(usage()).and_then(|a| a.deserialize()) {
        opts
    } else {
        println!("{}", usage());
        exit(1);
    }
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
    let opts = parse_opts();

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

    // Buffer stdout to improve performance.
    let stdout = io::BufWriter::new(io::stdout());

    match history.write(stdout) {
        Err(e) if e.kind() != ErrorKind::BrokenPipe => {
            eprintln!("Error: {}", e);
            exit(1);
        }
        _ => {
            exit(0);
        }
    }
}
