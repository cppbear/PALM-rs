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
#[derive(Clone)]
struct Transitions {
    /// The table.
    table: Vec<StatePtr>,
    /// The stride.
    num_byte_classes: usize,
}
impl Transitions {
    fn new(num_byte_classes: usize) -> Transitions {}
    fn num_states(&self) -> usize {}
    fn add(&mut self) -> Option<StatePtr> {
        let si = self.table.len();
        if si > STATE_MAX as usize {
            return None;
        }
        self.table.extend(repeat(STATE_UNKNOWN).take(self.num_byte_classes));
        Some(usize_to_u32(si))
    }
    fn clear(&mut self) {}
    fn set_next(&mut self, si: StatePtr, cls: usize, next: StatePtr) {}
    fn next(&self, si: StatePtr, cls: usize) -> StatePtr {}
    fn state_heap_size(&self) -> usize {}
    unsafe fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {}
}
fn usize_to_u32(n: usize) -> u32 {
    if (n as u64) > (::std::u32::MAX as u64) {
        panic!("BUG: {} is too big to fit into u32", n)
    }
    n as u32
}
