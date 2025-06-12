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
#[derive(Clone, Copy, Eq, Default, Hash, PartialEq)]
struct StateFlags(u8);
impl fmt::Debug for StateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StateFlags")
            .field("is_match", &self.is_match())
            .field("is_word", &self.is_word())
            .field("has_empty", &self.has_empty())
            .finish()
    }
}
impl StateFlags {
    fn is_match(&self) -> bool {
        self.0 & 0b0000000_1 > 0
    }
    fn set_match(&mut self) {}
    fn is_word(&self) -> bool {
        self.0 & 0b000000_1_0 > 0
    }
    fn set_word(&mut self) {}
    fn has_empty(&self) -> bool {
        self.0 & 0b00000_1_00 > 0
    }
    fn set_empty(&mut self) {}
}
