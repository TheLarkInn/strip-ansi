# strip-ansi
Rust inspired implementation of chalk/strip-ansi. 

## Usage
```rust 

use strip_ansi::strip_ansi;

fn main() {
    let string_with_colors: &str = "\u{001B}[0m\u{001B}[4m\u{001B}[42m\u{001B}[31mBetween \u{001B}[39m\u{001B}[49m\u{001B}[24mColors\u{001B}[0m";

    let uncolored_string = strip_ansi(string_with_colors);
}
```

