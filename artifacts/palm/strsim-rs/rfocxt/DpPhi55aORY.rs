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
pub fn normalized_damerau_levenshtein(a: &str, b: &str) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let len1 = a.chars().count();
    let len2 = b.chars().count();
    let dist = damerau_levenshtein_impl(a.chars(), len1, b.chars(), len2);
    1.0 - (dist as f64) / (max(len1, len2) as f64)
}
