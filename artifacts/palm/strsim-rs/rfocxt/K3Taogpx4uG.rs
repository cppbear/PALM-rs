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
pub fn sorensen_dice(a: &str, b: &str) -> f64 {
    let a: String = a.chars().filter(|&x| !char::is_whitespace(x)).collect();
    let b: String = b.chars().filter(|&x| !char::is_whitespace(x)).collect();
    if a == b {
        return 1.0;
    }
    if a.len() < 2 || b.len() < 2 {
        return 0.0;
    }
    let mut a_bigrams: HashMap<(char, char), usize> = HashMap::new();
    for bigram in bigrams(&a) {
        *a_bigrams.entry(bigram).or_insert(0) += 1;
    }
    let mut intersection_size = 0_usize;
    for bigram in bigrams(&b) {
        a_bigrams
            .entry(bigram)
            .and_modify(|bi| {
                if *bi > 0 {
                    *bi -= 1;
                    intersection_size += 1;
                }
            });
    }
    (2 * intersection_size) as f64 / (a.len() + b.len() - 2) as f64
}
fn bigrams(s: &str) -> impl Iterator<Item = (char, char)> + '_ {
    s.chars().zip(s.chars().skip(1))
}
