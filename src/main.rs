mod args;
mod strings;
mod utils;

use std::{ env, fs, io, process::exit };
use clap::Parser;
use args::Args;

#[cfg(unix)]
fn main() {
    let args = Args::parse();

    // Only print with dev profile.
    #[cfg(debug_assertions)]
    println!("{args:?}");

    // Please do this instead of using a match.
    let Ok(path_str) = env::var("PATH") else {
        eprintln!("No $PATH variable found.");
        exit(libc::EINVAL);
    };

    let mut result = Ok(libc::EXIT_SUCCESS);

    let colorize = if args.no_colors { false } else { args.colors.check_colorize() };

    let paths: Vec<&str> = path_str.split(':').collect();
    let num_padding = paths.len().to_string().len();

    for (i, path) in paths.iter().enumerate() {
        let mut output = String::new();

        if args.enumerate {
            // I wish there was a pad_left function.
            // I'm forced to use this weird fmt syntax.
            if args.zero_padding {
                output.push_str(&format!("{i:0>x$}: ", x = num_padding));
            } else {
                output.push_str(&format!("{i: >x$}: ", x = num_padding));
            }
        }

        if !args.no_status {            
            let (ok_str, err_str) = args.status_style
                .get_status_str(colorize);

            if !ok_str.is_empty() {
                // I'm actually proud of this statement.
                let status_str = match fs::metadata(path) {
                    Ok(m) if m.is_dir() => ok_str,
                    Ok(_) => {
                        result = Err(io::Error::from(io::ErrorKind::NotADirectory));
                        err_str
                    }
                    Err(e) => {
                        result = Err(e);
                        err_str
                    }
                };

                output.push_str(&status_str);
                output.push(' ');
            }
        }

        if args.no_quoting {
            output.push_str(path);
        } else {
            output.push_str(&format!("{path:?}"));
        }

        println!("{output}");
    }

    exit(result.unwrap_or_else(|e| e.raw_os_error().unwrap_or(1)))
}

#[cfg(windows)]
fn main() {
    compile_error!("Windows is not yet implemented.");
}

#[cfg(not(any(unix, windows)))]
fn main() {
    compile_error!("This program is only valid for POSIX or Windows systems.");
}
