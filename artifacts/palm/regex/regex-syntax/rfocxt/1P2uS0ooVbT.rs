use std::cmp;
use std::fmt;
use std::iter;
use std::mem;
use std::ops;
use hir::{self, Hir, HirKind};
fn position(needle: &[u8], mut haystack: &[u8]) -> Option<usize> {
    let mut i = 0;
    while haystack.len() >= needle.len() {
        if needle == &haystack[..needle.len()] {
            return Some(i);
        }
        i += 1;
        haystack = &haystack[1..];
    }
    None
}
