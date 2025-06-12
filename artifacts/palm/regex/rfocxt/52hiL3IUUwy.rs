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
#[allow(dead_code)]
fn show_state_ptr(si: StatePtr) -> String {
    let mut s = format!("{:?}", si & STATE_MAX);
    if si == STATE_UNKNOWN {
        s = format!("{} (unknown)", s);
    }
    if si == STATE_DEAD {
        s = format!("{} (dead)", s);
    }
    if si == STATE_QUIT {
        s = format!("{} (quit)", s);
    }
    if si & STATE_START > 0 {
        s = format!("{} (start)", s);
    }
    if si & STATE_MATCH > 0 {
        s = format!("{} (match)", s);
    }
    s
}
