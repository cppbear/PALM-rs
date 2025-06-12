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
fn bigrams(s: &str) -> impl Iterator<Item = (char, char)> + '_ {
    s.chars().zip(s.chars().skip(1))
}
