use super::Arg;

// `Arg::Help` is the last element of the `Arg` enum,
// thus `Arg::Help as usize + 1` means the element count of
// `Arg` enum.
pub const ARG_COUNT: usize = Arg::Help as usize + 1;
pub const ARGS: [[&str; 2]; ARG_COUNT] = [
    ["-t", "--timer"],
    ["-hf", "--headfile"],
    ["-e", "--editor"],
    ["-a", "--accent-color"],
    ["-i", "--indent-size"],
    ["-v", "--version"],
    ["-h", "--help"],
];
pub const ARG_DESCRIPTIONS: [&str; ARG_COUNT] = [
    "print extra execute duration message code execution.",
    "directly import variables in head files, must with script paths following.",
    "open build-in code editor",
    "editor accent color, options: [red, blue, dark_red, dark_blue, dark_grey, dark_cyan, dark_yellow, dark_magenta]",
    "editor indent size, default: 2",

    "print current executable file version and exit.",
    "print this help message.",
];

// defines the configurable args in the
// `config.calcrs` file.
pub(super) const ARG_CONFIGURABLE: [(&str, Arg); 4] = [
    ("timer", Arg::Timer),
    ("headfile", Arg::Headfile),
    ("accent_color", Arg::AccentColor),
    ("indent_size", Arg::IndentSize),
];
