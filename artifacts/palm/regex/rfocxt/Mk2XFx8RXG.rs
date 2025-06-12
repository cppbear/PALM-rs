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
fn read_vari32(data: &[u8]) -> (i32, usize) {
    let (un, i) = read_varu32(data);
    let mut n = (un >> 1) as i32;
    if un & 1 != 0 {
        n = !n;
    }
    (n, i)
}
fn read_varu32(data: &[u8]) -> (u32, usize) {
    let mut n: u32 = 0;
    let mut shift: u32 = 0;
    for (i, &b) in data.iter().enumerate() {
        if b < 0b1000_0000 {
            return (n | ((b as u32) << shift), i + 1);
        }
        n |= ((b as u32) & 0b0111_1111) << shift;
        shift += 7;
    }
    (0, 0)
}
