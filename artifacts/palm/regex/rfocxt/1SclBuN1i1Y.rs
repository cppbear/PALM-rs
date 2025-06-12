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
pub struct InstChar {
    /// The next location to execute in the program if this instruction
    /// succeeds.
    pub goto: InstPtr,
    /// The character to test.
    pub c: char,
}
#[derive(Clone, Debug)]
pub struct InstRanges {
    /// The next location to execute in the program if this instruction
    /// succeeds.
    pub goto: InstPtr,
    /// The set of Unicode scalar value ranges to test.
    pub ranges: Vec<(char, char)>,
}
#[derive(Clone, Debug)]
pub struct InstSplit {
    /// The first instruction to try. A match resulting from following goto1
    /// has precedence over a match resulting from following goto2.
    pub goto1: InstPtr,
    /// The second instruction to try. A match resulting from following goto1
    /// has precedence over a match resulting from following goto2.
    pub goto2: InstPtr,
}
#[derive(Clone, Debug)]
pub struct InstBytes {
    /// The next location to execute in the program if this instruction
    /// succeeds.
    pub goto: InstPtr,
    /// The start (inclusive) of this byte range.
    pub start: u8,
    /// The end (inclusive) of this byte range.
    pub end: u8,
}
#[derive(Clone, Debug)]
pub struct InstSave {
    /// The next location to execute in the program.
    pub goto: InstPtr,
    /// The capture slot (there are two slots for every capture in a regex,
    /// including the zeroth capture for the entire match).
    pub slot: usize,
}
#[derive(Clone, Debug)]
pub struct InstEmptyLook {
    /// The next location to execute in the program if this instruction
    /// succeeds.
    pub goto: InstPtr,
    /// The type of zero-width assertion to check.
    pub look: EmptyLook,
}
#[derive(Clone, Debug)]
pub enum Inst {
    /// Match indicates that the program has reached a match state.
    ///
    /// The number in the match corresponds to the Nth logical regular
    /// expression in this program. This index is always 0 for normal regex
    /// programs. Values greater than 0 appear when compiling regex sets, and
    /// each match instruction gets its own unique value. The value corresponds
    /// to the Nth regex in the set.
    Match(usize),
    /// Save causes the program to save the current location of the input in
    /// the slot indicated by InstSave.
    Save(InstSave),
    /// Split causes the program to diverge to one of two paths in the
    /// program, preferring goto1 in InstSplit.
    Split(InstSplit),
    /// EmptyLook represents a zero-width assertion in a regex program. A
    /// zero-width assertion does not consume any of the input text.
    EmptyLook(InstEmptyLook),
    /// Char requires the regex program to match the character in InstChar at
    /// the current position in the input.
    Char(InstChar),
    /// Ranges requires the regex program to match the character at the current
    /// position in the input with one of the ranges specified in InstRanges.
    Ranges(InstRanges),
    /// Bytes is like Ranges, except it expresses a single byte range. It is
    /// used in conjunction with Split instructions to implement multi-byte
    /// character classes.
    Bytes(InstBytes),
}
impl Inst {
    pub fn is_match(&self) -> bool {
        match *self {
            Inst::Match(_) => true,
            _ => false,
        }
    }
}
