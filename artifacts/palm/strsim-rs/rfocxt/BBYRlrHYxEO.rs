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
impl Display for StrSimError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let text = match self {
            StrSimError::DifferentLengthArgs => "Differing length arguments provided",
        };
        write!(fmt, "{}", text)
    }
}
