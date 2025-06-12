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
#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    data: Box<[u8]>,
}
#[derive(Clone, Copy, Eq, Default, Hash, PartialEq)]
struct StateFlags(u8);
impl State {
    fn flags(&self) -> StateFlags {
        StateFlags(self.data[0])
    }
    fn inst_ptrs(&self) -> InstPtrs {}
}
