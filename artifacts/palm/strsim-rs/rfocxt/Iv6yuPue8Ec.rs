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
pub fn damerau_levenshtein(a: &str, b: &str) -> usize {
    damerau_levenshtein_impl(a.chars(), a.chars().count(), b.chars(), b.chars().count())
}
