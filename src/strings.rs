pub const DEFAULT_FORMAT: &str = "({index:>}: )({status:-} ){path:<}( {description})";

pub const AFTER_HELP: &str = r#"
This program supports NO_COLOR (see https://nocolor.org)

The <FORMAT> argument defines a custom output format using
placeholders and simple formatting rules. It may contain
variables written as {name}. Each variable is later expanded
with its corresponding value, and may be not present.
Variables can include optional alignment by using the
syntax "{name:xy}", where "x" is the alignment specifier
and "y" is the filler character.
For example, "{status:>-}" aligns the value of the "status"
variable to the right using the "-" character.
Parentheses can be used for conditional sections. Any text
inside is only included if the variable inside is not empty,
otherwise the entire section is removed.
Multiple variable expansions inside a single section are not
allowed.
Parentheses without a variable expansion inside are ignored.

Variable names:
    index          When the --enumerate flag is present,
                   it's the current line index (starting
                   from 1)
    path           The path text
    status         The status symbol of the path
    description    Current status of the path

Alignment specifiers:
    <    Align to the left
    >    Align to the right
    -    Center the text

The default character for padding is space (\u{0020}).

Example:
    # Display paths by padding numbers to the
    # right and centering the path.
    pathcheck -e --format="({line:>} - ){path:>} - {description}"

Note: header labels are affected by the format specified by the
      --format flag.
"#;

#[derive(Clone)]
pub struct StatusSet {
    pub ok: &'static str,
    pub error: &'static str,
    pub warning: &'static str,
}

macro_rules! define_sets {
    ($($name:ident = { $ok:literal, $error:literal, $warning:literal })*) => {
        $(
            pub const $name: StatusSet = StatusSet {
                ok: $ok,
                error: $error,
                warning: $warning
            };
        )*
    };
}

define_sets! {
     ICON_SET = { "\u{f058}", "\u{f530}", "\u{f071}" }
    EMOJI_SET = { "\u{2705}", "\u{274c}", "\u{26a0}" }
     TEXT_SET = { "OK ", "ERR", "(!)" }
}
