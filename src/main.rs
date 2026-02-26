// Can't define default_value_t otherwise.
#![allow(unused_parens)]

use std::{ env, fs, process::ExitCode };
use clap::Parser;
use owo_colors::OwoColorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short='a')]
    a: bool,
    #[arg(short = 's', long = "style", default_value_t = ("numbers,status".to_owned()))]
    style: String
}

const OK_TEXT: &str = "\u{f058}";
const ERR_TEXT: &str = "\u{f530}";
const UNKNOWN_TEXT: &str = "\u{f071}";

fn main() -> ExitCode {
    let args = Args::parse();

    // Only print with dev profile.
    #[cfg(debug_assertions)]
    println!("{args:?}");

    let mut status_text = UNKNOWN_TEXT.yellow().to_string();
    let Ok(path_str) = env::var("PATH") else {
        eprintln!("No $PATH variable found.");
        return ExitCode::FAILURE;
    };
    let paths: Vec<&str> = path_str.split(':').collect();
    for (i, path) in paths.iter().enumerate() {
        change_status(&mut status_text, fs::exists(path));
        change_status(
            &mut status_text,
            fs::File::open(path)
                .and_then(|file| file.metadata())
                .map(|meta| meta.is_dir())
        );
        println!("{:>2 }: {} {}", i + 1, path, status_text);
    }
    ExitCode::SUCCESS
}

fn change_status<E>(text: &mut String, result: Result<bool, E>) {
    if let Ok(does) = result {
        *text = if does { OK_TEXT.green().to_string() } else { ERR_TEXT.red().to_string() };
    }
}
