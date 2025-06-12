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
pub fn osa_distance(a: &str, b: &str) -> usize {
    let b_len = b.chars().count();
    let mut prev_two_distances: Vec<usize> = (0..b_len + 1).collect();
    let mut prev_distances: Vec<usize> = (0..b_len + 1).collect();
    let mut curr_distances: Vec<usize> = vec![0; b_len + 1];
    let mut prev_a_char = char::MAX;
    let mut prev_b_char = char::MAX;
    for (i, a_char) in a.chars().enumerate() {
        curr_distances[0] = i + 1;
        for (j, b_char) in b.chars().enumerate() {
            let cost = usize::from(a_char != b_char);
            curr_distances[j + 1] = min(
                curr_distances[j] + 1,
                min(prev_distances[j + 1] + 1, prev_distances[j] + cost),
            );
            if i > 0 && j > 0 && a_char != b_char && a_char == prev_b_char
                && b_char == prev_a_char
            {
                curr_distances[j + 1] = min(
                    curr_distances[j + 1],
                    prev_two_distances[j - 1] + 1,
                );
            }
            prev_b_char = b_char;
        }
        mem::swap(&mut prev_two_distances, &mut prev_distances);
        mem::swap(&mut prev_distances, &mut curr_distances);
        prev_a_char = a_char;
    }
    prev_distances[b_len]
}
