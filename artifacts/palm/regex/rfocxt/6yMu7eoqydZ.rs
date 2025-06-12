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
struct InstPtrs<'a> {
    base: usize,
    data: &'a [u8],
}
impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ips: Vec<usize> = self.inst_ptrs().collect();
        f.debug_struct("State")
            .field("flags", &self.flags())
            .field("insts", &ips)
            .finish()
    }
}
impl State {
    fn flags(&self) -> StateFlags {
        StateFlags(self.data[0])
    }
    fn inst_ptrs(&self) -> InstPtrs {
        InstPtrs {
            base: 0,
            data: &self.data[1..],
        }
    }
}
