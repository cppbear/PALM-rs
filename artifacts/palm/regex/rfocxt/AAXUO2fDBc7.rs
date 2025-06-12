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
struct Parsed {
    exprs: Vec<Hir>,
    prefixes: Literals,
    suffixes: Literals,
    bytes: bool,
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
#[derive(Clone, PartialEq)]
pub enum Error {
    /// A syntax error.
    Syntax(String),
    /// The compiled program exceeded the set size limit.
    /// The argument is the size limit imposed.
    CompiledTooBig(usize),
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
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
    fn parse(&self) -> Result<Parsed, Error> {
        let mut exprs = Vec::with_capacity(self.options.pats.len());
        let mut prefixes = Some(Literals::empty());
        let mut suffixes = Some(Literals::empty());
        let mut bytes = false;
        let is_set = self.options.pats.len() > 1;
        for pat in &self.options.pats {
            let mut parser = ParserBuilder::new()
                .octal(self.options.octal)
                .case_insensitive(self.options.case_insensitive)
                .multi_line(self.options.multi_line)
                .dot_matches_new_line(self.options.dot_matches_new_line)
                .swap_greed(self.options.swap_greed)
                .ignore_whitespace(self.options.ignore_whitespace)
                .unicode(self.options.unicode)
                .allow_invalid_utf8(!self.only_utf8)
                .nest_limit(self.options.nest_limit)
                .build();
            let expr = parser.parse(pat).map_err(|e| Error::Syntax(e.to_string()))?;
            bytes = bytes || !expr.is_always_utf8();
            if !expr.is_anchored_start() && expr.is_any_anchored_start() {
                prefixes = None;
            } else if is_set && expr.is_anchored_start() {
                prefixes = None;
            }
            prefixes = prefixes
                .and_then(|mut prefixes| {
                    if !prefixes.union_prefixes(&expr) { None } else { Some(prefixes) }
                });
            if !expr.is_anchored_end() && expr.is_any_anchored_end() {
                suffixes = None;
            } else if is_set && expr.is_anchored_end() {
                suffixes = None;
            }
            suffixes = suffixes
                .and_then(|mut suffixes| {
                    if !suffixes.union_suffixes(&expr) { None } else { Some(suffixes) }
                });
            exprs.push(expr);
        }
        Ok(Parsed {
            exprs: exprs,
            prefixes: prefixes.unwrap_or_else(Literals::empty),
            suffixes: suffixes.unwrap_or_else(Literals::empty),
            bytes: bytes,
        })
    }
    pub fn build(self) -> Result<Exec, Error> {}
}
