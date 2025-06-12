type Result = result::Result<Patch, Error>;
use std::collections::HashMap;
use std::iter;
use std::result;
use std::sync::Arc;
use syntax::is_word_byte;
use syntax::hir::{self, Hir};
use utf8_ranges::{Utf8Range, Utf8Sequence, Utf8Sequences};
use prog::{
    Program, Inst, InstPtr, EmptyLook, InstSave, InstSplit, InstEmptyLook, InstChar,
    InstRanges, InstBytes,
};
use Error;
#[derive(Clone, Debug)]
pub struct InstSplit {
    /// The first instruction to try. A match resulting from following goto1
    /// has precedence over a match resulting from following goto2.
    pub goto1: InstPtr,
    /// The second instruction to try. A match resulting from following goto1
    /// has precedence over a match resulting from following goto2.
    pub goto2: InstPtr,
}
#[derive(Clone, Debug)]
enum MaybeInst {
    Compiled(Inst),
    Uncompiled(InstHole),
    Split,
    Split1(InstPtr),
    Split2(InstPtr),
}
#[derive(Clone, Debug)]
enum InstHole {
    Save { slot: usize },
    EmptyLook { look: EmptyLook },
    Char { c: char },
    Ranges { ranges: Vec<(char, char)> },
    Bytes { start: u8, end: u8 },
}
#[derive(Clone, Debug)]
pub enum Inst {
    /// Match indicates that the program has reached a match state.
    ///
    /// The number in the match corresponds to the Nth logical regular
    /// expression in this program. This index is always 0 for normal regex
    /// programs. Values greater than 0 appear when compiling regex sets, and
    /// each match instruction gets its own unique value. The value corresponds
    /// to the Nth regex in the set.
    Match(usize),
    /// Save causes the program to save the current location of the input in
    /// the slot indicated by InstSave.
    Save(InstSave),
    /// Split causes the program to diverge to one of two paths in the
    /// program, preferring goto1 in InstSplit.
    Split(InstSplit),
    /// EmptyLook represents a zero-width assertion in a regex program. A
    /// zero-width assertion does not consume any of the input text.
    EmptyLook(InstEmptyLook),
    /// Char requires the regex program to match the character in InstChar at
    /// the current position in the input.
    Char(InstChar),
    /// Ranges requires the regex program to match the character at the current
    /// position in the input with one of the ranges specified in InstRanges.
    Ranges(InstRanges),
    /// Bytes is like Ranges, except it expresses a single byte range. It is
    /// used in conjunction with Split instructions to implement multi-byte
    /// character classes.
    Bytes(InstBytes),
}
impl MaybeInst {
    fn fill(&mut self, goto: InstPtr) {
        let filled = match *self {
            MaybeInst::Uncompiled(ref inst) => inst.fill(goto),
            MaybeInst::Split1(goto1) => {
                Inst::Split(InstSplit {
                    goto1: goto1,
                    goto2: goto,
                })
            }
            MaybeInst::Split2(goto2) => {
                Inst::Split(InstSplit {
                    goto1: goto,
                    goto2: goto2,
                })
            }
            _ => {
                unreachable!(
                    "not all instructions were compiled! \
                               found uncompiled instruction: {:?}",
                    self
                )
            }
        };
        *self = MaybeInst::Compiled(filled);
    }
    fn fill_split(&mut self, goto1: InstPtr, goto2: InstPtr) {}
    fn half_fill_split_goto1(&mut self, goto1: InstPtr) {}
    fn half_fill_split_goto2(&mut self, goto2: InstPtr) {}
    fn unwrap(self) -> Inst {}
}
impl InstHole {
    fn fill(&self, goto: InstPtr) -> Inst {
        match *self {
            InstHole::Save { slot } => Inst::Save(InstSave { goto: goto, slot: slot }),
            InstHole::EmptyLook { look } => {
                Inst::EmptyLook(InstEmptyLook {
                    goto: goto,
                    look: look,
                })
            }
            InstHole::Char { c } => Inst::Char(InstChar { goto: goto, c: c }),
            InstHole::Ranges { ref ranges } => {
                Inst::Ranges(InstRanges {
                    goto: goto,
                    ranges: ranges.clone(),
                })
            }
            InstHole::Bytes { start, end } => {
                Inst::Bytes(InstBytes {
                    goto: goto,
                    start: start,
                    end: end,
                })
            }
        }
    }
}
