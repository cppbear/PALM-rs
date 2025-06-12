type InstPtr = u32;
type StatePtr = u32;
use std::collections::HashMap;
use std::fmt;
use std::iter::repeat;
use std::mem;
use exec::ProgramCache;
use prog::{Inst, Program};
use sparse::SparseSet;
const STATE_UNKNOWN: StatePtr = 1 << 31;
const STATE_DEAD: StatePtr = STATE_UNKNOWN + 1;
const STATE_QUIT: StatePtr = STATE_DEAD + 1;
const STATE_START: StatePtr = 1 << 30;
const STATE_MATCH: StatePtr = 1 << 29;
const STATE_MAX: StatePtr = STATE_MATCH - 1;
#[derive(Clone, Debug)]
pub enum Result<T> {
    Match(T),
    NoMatch(usize),
    Quit,
}
impl<T> Result<T> {
    pub fn is_match(&self) -> bool {}
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Result<U> {
        match self {
            Result::Match(t) => Result::Match(f(t)),
            Result::NoMatch(x) => Result::NoMatch(x),
            Result::Quit => Result::Quit,
        }
    }
    fn set_non_match(self, at: usize) -> Result<T> {}
}
