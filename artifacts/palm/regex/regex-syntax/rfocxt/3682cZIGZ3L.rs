pub use error::{Error, Result};
pub use parser::{Parser, ParserBuilder};
pub fn escape_into(text: &str, buf: &mut String) {
    for c in text.chars() {
        if is_meta_character(c) {
            buf.push('\\');
        }
        buf.push(c);
    }
}
pub fn is_meta_character(c: char) -> bool {
    match c {
        '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^'
        | '$' | '#' | '&' | '-' | '~' => true,
        _ => false,
    }
}
