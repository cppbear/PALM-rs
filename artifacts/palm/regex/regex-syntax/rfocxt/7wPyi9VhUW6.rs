pub use error::{Error, Result};
pub use parser::{Parser, ParserBuilder};
pub fn escape(text: &str) -> String {
    let mut quoted = String::with_capacity(text.len());
    escape_into(text, &mut quoted);
    quoted
}
pub fn escape_into(text: &str, buf: &mut String) {
    for c in text.chars() {
        if is_meta_character(c) {
            buf.push('\\');
        }
        buf.push(c);
    }
}
