pub type HammingResult = Result<usize, StrSimError>;
use std::char;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
use std::mem;
use std::str::Chars;
#[derive(Debug, PartialEq)]
pub enum StrSimError {
    DifferentLengthArgs,
}
pub fn hamming(a: &str, b: &str) -> HammingResult {
    generic_hamming(a.chars(), b.chars())
}
