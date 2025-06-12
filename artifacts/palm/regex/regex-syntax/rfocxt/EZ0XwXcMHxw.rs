use std::cmp;
use std::fmt;
use std::iter;
use std::mem;
use std::ops;
use hir::{self, Hir, HirKind};
fn escape_bytes(bytes: &[u8]) -> String {
    let mut s = String::new();
    for &b in bytes {
        s.push_str(&escape_byte(b));
    }
    s
}
fn escape_byte(byte: u8) -> String {
    use std::ascii::escape_default;
    let escaped: Vec<u8> = escape_default(byte).collect();
    String::from_utf8_lossy(&escaped).into_owned()
}
