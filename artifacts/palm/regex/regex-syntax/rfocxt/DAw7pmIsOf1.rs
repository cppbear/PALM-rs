pub use error::{Error, Result};
pub use parser::{Parser, ParserBuilder};
pub fn is_meta_character(c: char) -> bool {
    match c {
        '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^'
        | '$' | '#' | '&' | '-' | '~' => true,
        _ => false,
    }
}
