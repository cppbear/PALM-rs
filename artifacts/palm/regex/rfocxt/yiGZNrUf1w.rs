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
    fn add(&mut self) -> Option<StatePtr> {}
    fn clear(&mut self) {
        self.table.clear();
    }
    fn set_next(&mut self, si: StatePtr, cls: usize, next: StatePtr) {}
    fn next(&self, si: StatePtr, cls: usize) -> StatePtr {}
    fn state_heap_size(&self) -> usize {}
    unsafe fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {}
}
