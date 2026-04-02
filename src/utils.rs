use crate::args::WhenColors;
use std::fmt::Display;
use lazy_regex::regex_remove_all;
use owo_colors::{ OwoColorize, Stream as OwoStream };

pub fn fatal<D: Display>(msg: D) -> ! {
    eprintln!("Error: {msg}");
    std::process::exit(libc::EINVAL);
}


pub fn visible_len<T: AsRef<str>>(val: T) -> usize {
    regex_remove_all!(
        r"\x1b\[(\d+;)*\d*;?[a-zA-Z]",
        val.as_ref()
    ).into_owned().chars().count()
}

macro_rules! define_color_fn {
    ($($color:ident),+) => { $(
        pub fn $color(text: &str, color: WhenColors) -> String {
            match color {
                WhenColors::Never => text.to_string(),
                WhenColors::Always => text.$color().to_string(),
                WhenColors::Auto => {
                    text.if_supports_color(
                        OwoStream::Stdout,
                        OwoColorize::$color
                    ).to_string()
                }
            }
        }
    )+ };
}

define_color_fn!(green, red, yellow);
