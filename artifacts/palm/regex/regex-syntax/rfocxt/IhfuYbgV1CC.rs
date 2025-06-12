pub type Result<T> = result::Result<T, Error>;
use std::cmp;
use std::error;
use std::fmt;
use std::result;
use ast;
use hir;
fn repeat_char(c: char, count: usize) -> String {
    ::std::iter::repeat(c).take(count).collect()
}
