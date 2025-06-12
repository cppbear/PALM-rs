pub type InstPtr = usize;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;
use std::ops::Deref;
use std::mem;
use std::slice;
use std::sync::Arc;
use input::Char;
use literal::LiteralSearcher;
#[derive(Clone, Debug)]
pub struct InstRanges {
    /// The next location to execute in the program if this instruction
    /// succeeds.
    pub goto: InstPtr,
    /// The set of Unicode scalar value ranges to test.
    pub ranges: Vec<(char, char)>,
}
impl InstRanges {
    pub fn matches(&self, c: Char) -> bool {}
    pub fn num_chars(&self) -> usize {
        self
            .ranges
            .iter()
            .map(|&(s, e)| 1 + (e as u32) - (s as u32))
            .fold(0, |acc, len| acc + len) as usize
    }
}
