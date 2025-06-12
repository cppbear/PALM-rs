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
fn vb(b: usize) -> String {
    use std::ascii::escape_default;
    if b > ::std::u8::MAX as usize {
        "EOF".to_owned()
    } else {
        let escaped = escape_default(b as u8).collect::<Vec<u8>>();
        String::from_utf8_lossy(&escaped).into_owned()
    }
}
