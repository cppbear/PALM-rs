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
fn push_inst_ptr(data: &mut Vec<u8>, prev: &mut InstPtr, ip: InstPtr) {
    let delta = (ip as i32) - (*prev as i32);
    write_vari32(data, delta);
    *prev = ip;
}
fn write_vari32(data: &mut Vec<u8>, n: i32) {
    let mut un = (n as u32) << 1;
    if n < 0 {
        un = !un;
    }
    write_varu32(data, un)
}
