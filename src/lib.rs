extern crate ansi_regex;
use ansi_regex::ansi_regex;

pub fn strip_ansi(string: &str) -> String {
    // Strip ANSI codes from a string
    ansi_regex().replace_all(string, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi_basic() {
        let string = "Hello \x1b[1mWorld\x1b[0m!";
        let stripped_string = "Hello World!";

        assert_eq!(strip_ansi(string), stripped_string);
    }

    #[test]
    fn strip_color_from_string() {
        let string = "\u{001B}[0m\u{001B}[4m\u{001B}[42m\u{001B}[31mBetween \u{001B}[39m\u{001B}[49m\u{001B}[24mColors\u{001B}[0m";
        let stripped_string = "Between Colors";

        assert_eq!(strip_ansi(string), stripped_string);
    }

    #[test]
    fn strip_color_from_ls_command() {
        let string = "\u{001B}[00;38;5;244m\u{001B}[m\u{001B}[00;38;5;33mls command\u{001B}[0m";
        let stripped_string = "ls command";

        assert_eq!(strip_ansi(string), stripped_string);
    }

    #[test]
    fn strip_reset_setfg_setbg_italics_strike_underline_sequence_from_string() {
        let string = "\u{001B}[0;33;49;3;9;4mformatters\u{001B}[0m";
        let stripped_string = "formatters";

        assert_eq!(strip_ansi(string), stripped_string);
    }

    #[test]
    fn strip_link_from_terminal_link() {
        let string = "\u{001B}]8;;https://github.com\u{0007}click\u{001B}]8;;\u{0007}";
        let stripped_string = "click";

        assert_eq!(strip_ansi(string), stripped_string);
    }
}
