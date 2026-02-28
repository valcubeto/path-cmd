use crate::strings::*;
use crate::utils::stdout_is_tty;
use clap::{ Parser, ValueEnum };
use owo_colors::OwoColorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "test")]
pub struct Args {
    /// Enumerate paths.
    #[arg(short = 'e', long = "enumerate")]
    pub enumerate: bool,

    /// Pad numbers with zeroes instead of spaces.
    #[arg(short = 'z', long = "zero-padding")]
    pub zero_padding: bool,

    /// Disable path quoting.
    #[arg(short = 'Q', long = "no-quotes")]
    pub no_quoting: bool,

    /// Do not perform status calculations.
    #[arg(short = 'S', long = "no-status")]
    pub no_status: bool,

    #[arg(
        value_enum,
        short = 's',
        long = "status-style",
        default_value_t = StatusStyle::Text
    )]
    pub status_style: StatusStyle,

    /// Disable colors
    #[arg(short = 'C', long = "no-colors")]
    pub no_colors: bool,

    #[arg(
        value_enum,
        short = 'c',
        long = "colors",
        default_value_t = WhenColors::Auto
    )]
    pub colors: WhenColors
}

#[derive(ValueEnum, Debug, Clone)]
pub enum StatusStyle {
    NerdFont,
    Emoji,
    Text,
    None,
}

impl StatusStyle {
    pub fn get_status_str(&self, colorize: bool) -> (String, String) {
        match self {
            StatusStyle::NerdFont if colorize => (
                NF_OK.green().to_string(),
                NF_ERR.red().to_string()
            ),
            StatusStyle::NerdFont => (
                NF_OK.into(),
                NF_ERR.into()
            ),
            StatusStyle::Text if colorize => (
                TEXT_OK.green().to_string(),
                TEXT_ERR.red().to_string()
            ),
            StatusStyle::Text => (
                TEXT_OK.into(),
                TEXT_ERR.into()
            ),
            StatusStyle::Emoji => (
                EMOJI_OK.into(),
                EMOJI_ERR.into()
            ),
            StatusStyle::None => (
                String::new(),
                String::new()
            )
        }
    }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum WhenColors {
    Always,
    Auto,
    Never,
}

impl WhenColors {
    pub fn check_colorize(&self) -> bool {
        match self {
            WhenColors::Always => true,
            WhenColors::Never => false,
            WhenColors::Auto => stdout_is_tty()
        }
    }
}
