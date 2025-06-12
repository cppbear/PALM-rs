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
pub fn jaro(a: &str, b: &str) -> f64 {
    generic_jaro(&StringWrapper(a), &StringWrapper(b))
}
