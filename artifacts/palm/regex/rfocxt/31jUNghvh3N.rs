pub type ProgramCache = RefCell<ProgramCacheInner>;
use std::cell::RefCell;
use std::collections::HashMap;
use std::cmp;
use std::sync::Arc;
use thread_local::CachedThreadLocal;
use syntax::ParserBuilder;
use syntax::hir::Hir;
use syntax::hir::literal::Literals;
use backtrack;
use compile::Compiler;
use dfa;
use error::Error;
use input::{ByteInput, CharInput};
use literal::LiteralSearcher;
use pikevm;
use prog::Program;
use re_builder::RegexOptions;
use re_bytes;
use re_set;
use re_trait::{RegularExpression, Slot, Locations, as_slots};
use re_unicode;
use utf8::next_utf8;
pub trait RegularExpression: Sized {
    type Text: ?Sized;
    fn slots_len(&self) -> usize;
    fn locations(&self) -> Locations;
    fn next_after_empty(&self, text: &Self::Text, i: usize) -> usize;
    fn shortest_match_at(&self, text: &Self::Text, start: usize) -> Option<usize>;
    fn is_match_at(&self, text: &Self::Text, start: usize) -> bool;
    fn find_at(&self, text: &Self::Text, start: usize) -> Option<(usize, usize)>;
    fn read_captures_at(
        &self,
        locs: &mut Locations,
        text: &Self::Text,
        start: usize,
    ) -> Option<(usize, usize)>;
    fn find_iter(self, text: &Self::Text) -> Matches<Self> {
        Matches {
            re: self,
            text: text,
            last_end: 0,
            last_match: None,
        }
    }
    fn captures_iter(self, text: &Self::Text) -> CaptureMatches<Self> {
        CaptureMatches(self.find_iter(text))
    }
}
pub struct ExecNoSyncStr<'c>(ExecNoSync<'c>);
#[derive(Debug)]
pub struct ExecNoSync<'c> {
    /// All read only state.
    ro: &'c Arc<ExecReadOnly>,
    /// Caches for the various matching engines.
    cache: &'c ProgramCache,
}
impl<'c> RegularExpression for ExecNoSyncStr<'c> {
    type Text = str;
    fn slots_len(&self) -> usize {}
    fn next_after_empty(&self, text: &str, i: usize) -> usize {}
    #[inline(always)]
    fn shortest_match_at(&self, text: &str, start: usize) -> Option<usize> {
        self.0.shortest_match_at(text.as_bytes(), start)
    }
    #[inline(always)]
    fn is_match_at(&self, text: &str, start: usize) -> bool {}
    #[inline(always)]
    fn find_at(&self, text: &str, start: usize) -> Option<(usize, usize)> {}
    #[inline(always)]
    fn read_captures_at(
        &self,
        locs: &mut Locations,
        text: &str,
        start: usize,
    ) -> Option<(usize, usize)> {}
}
