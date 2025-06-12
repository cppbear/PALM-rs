use std::cmp;
use std::fmt;
use std::iter;
use std::mem;
use std::ops;
use hir::{self, Hir, HirKind};
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
fn escape_byte(byte: u8) -> String {
    use std::ascii::escape_default;
    let escaped: Vec<u8> = escape_default(byte).collect();
    String::from_utf8_lossy(&escaped).into_owned()
}
fn escape_bytes(bytes: &[u8]) -> String {
    let mut s = String::new();
    for &b in bytes {
        s.push_str(&escape_byte(b));
    }
    s
}
