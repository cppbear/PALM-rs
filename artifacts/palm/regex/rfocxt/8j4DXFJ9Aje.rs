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
struct SuffixCache {
    table: Vec<SuffixCacheEntry>,
    version: usize,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct SuffixCacheEntry {
    key: SuffixCacheKey,
    pc: InstPtr,
    version: usize,
}
impl SuffixCache {
    fn new(size: usize) -> Self {
        SuffixCache {
            table: vec![SuffixCacheEntry::default(); size],
            version: 0,
        }
    }
    fn get(&mut self, key: SuffixCacheKey, pc: InstPtr) -> Option<InstPtr> {}
    fn clear(&mut self) {
        self.version += 1;
    }
    fn hash(&self, suffix: &SuffixCacheKey) -> usize {}
}
