use std::cmp;
use std::fmt;
use std::iter;
use std::mem;
use std::ops;
use hir::{self, Hir, HirKind};
#[derive(Clone, Eq, Ord)]
pub struct Literal {
    v: Vec<u8>,
    cut: bool,
}
impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_cut() {
            write!(f, "Cut({})", escape_unicode(& self.v))
        } else {
            write!(f, "Complete({})", escape_unicode(& self.v))
        }
    }
}
impl Literal {
    pub fn new(bytes: Vec<u8>) -> Literal {}
    pub fn empty() -> Literal {}
    pub fn is_cut(&self) -> bool {
        self.cut
    }
    pub fn cut(&mut self) {}
}
fn escape_unicode(bytes: &[u8]) -> String {
    let show = match ::std::str::from_utf8(bytes) {
        Ok(v) => v.to_string(),
        Err(_) => escape_bytes(bytes),
    };
    let mut space_escaped = String::new();
    for c in show.chars() {
        if c.is_whitespace() {
            let escaped = if c as u32 <= 0x7F {
                escape_byte(c as u8)
            } else {
                if c as u32 <= 0xFFFF {
                    format!(r"\u{{{:04x}}}", c as u32)
                } else {
                    format!(r"\U{{{:08x}}}", c as u32)
                }
            };
            space_escaped.push_str(&escaped);
        } else {
            space_escaped.push(c);
        }
    }
    space_escaped
}
