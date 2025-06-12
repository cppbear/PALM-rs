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
struct ByteClassSet([bool; 256]);
impl ByteClassSet {
    fn new() -> Self {
        ByteClassSet([false; 256])
    }
    fn set_range(&mut self, start: u8, end: u8) {}
    fn set_word_boundary(&mut self) {}
    fn byte_classes(&self) -> Vec<u8> {}
}
