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
struct StringWrapper<'a>(&'a str);
pub fn levenshtein(a: &str, b: &str) -> usize {
    generic_levenshtein(&StringWrapper(a), &StringWrapper(b))
}
