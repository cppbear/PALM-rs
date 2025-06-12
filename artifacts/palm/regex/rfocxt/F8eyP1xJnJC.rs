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
pub struct ExecBuilder {
    options: RegexOptions,
    match_type: Option<MatchType>,
    bytes: bool,
    only_utf8: bool,
}
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct RegexOptions {
    pub pats: Vec<String>,
    pub size_limit: usize,
    pub dfa_size_limit: usize,
    pub nest_limit: u32,
    pub case_insensitive: bool,
    pub multi_line: bool,
    pub dot_matches_new_line: bool,
    pub swap_greed: bool,
    pub ignore_whitespace: bool,
    pub unicode: bool,
    pub octal: bool,
}
#[derive(Clone, Copy, Debug)]
enum MatchType {
    /// A single or multiple literal search. This is only used when the regex
    /// can be decomposed into unambiguous literal search.
    Literal(MatchLiteralType),
    /// A normal DFA search.
    Dfa,
    /// A reverse DFA search starting from the end of a haystack.
    DfaAnchoredReverse,
    /// A reverse DFA search with suffix literal scanning.
    DfaSuffix,
    /// Use the DFA on two or more regular expressions.
    DfaMany,
    /// An NFA variant.
    Nfa(MatchNfaType),
    /// No match is ever possible, so don't ever try to search.
    Nothing,
}
#[derive(Clone, Copy, Debug)]
enum MatchNfaType {
    /// Choose between Backtrack and PikeVM.
    Auto,
    /// NFA bounded backtracking.
    ///
    /// (This is only set by tests, since it never makes sense to always want
    /// backtracking.)
    Backtrack,
    /// The Pike VM.
    ///
    /// (This is only set by tests, since it never makes sense to always want
    /// the Pike VM.)
    PikeVM,
}
impl ExecBuilder {
    pub fn new(re: &str) -> Self {
        Self::new_many(&[re])
    }
    pub fn new_many<I, S>(res: I) -> Self
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let mut opts = RegexOptions::default();
        opts.pats = res.into_iter().map(|s| s.as_ref().to_owned()).collect();
        Self::new_options(opts)
    }
    pub fn new_options(opts: RegexOptions) -> Self {
        ExecBuilder {
            options: opts,
            match_type: None,
            bytes: false,
            only_utf8: true,
        }
    }
    pub fn automatic(mut self) -> Self {
        self.match_type = None;
        self
    }
    pub fn nfa(mut self) -> Self {
        self.match_type = Some(MatchType::Nfa(MatchNfaType::PikeVM));
        self
    }
    pub fn bounded_backtracking(mut self) -> Self {
        self.match_type = Some(MatchType::Nfa(MatchNfaType::Backtrack));
        self
    }
    pub fn bytes(mut self, yes: bool) -> Self {
        self.bytes = yes;
        self
    }
    pub fn only_utf8(mut self, yes: bool) -> Self {
        self.only_utf8 = yes;
        self
    }
    pub fn unicode(mut self, yes: bool) -> Self {
        self.options.unicode = yes;
        self
    }
    fn parse(&self) -> Result<Parsed, Error> {}
    pub fn build(self) -> Result<Exec, Error> {}
}
