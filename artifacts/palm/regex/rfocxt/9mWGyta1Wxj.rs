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
#[derive(Copy, Clone, Debug)]
struct Byte(u16);
impl Byte {
    fn byte(b: u8) -> Self {
        Byte(b as u16)
    }
    fn eof() -> Self {
        Byte(256)
    }
    fn is_eof(&self) -> bool {
        self.0 == 256
    }
    fn is_ascii_word(&self) -> bool {}
    fn as_byte(&self) -> Option<u8> {
        if self.is_eof() { None } else { Some(self.0 as u8) }
    }
}
