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
pub struct Exec {
    /// All read only state.
    ro: Arc<ExecReadOnly>,
    /// Caches for the various matching engines.
    cache: CachedThreadLocal<ProgramCache>,
}
#[derive(Debug)]
struct ExecReadOnly {
    /// The original regular expressions given by the caller to compile.
    res: Vec<String>,
    /// A compiled program that is used in the NFA simulation and backtracking.
    /// It can be byte-based or Unicode codepoint based.
    ///
    /// N.B. It is not possibly to make this byte-based from the public API.
    /// It is only used for testing byte based programs in the NFA simulations.
    nfa: Program,
    /// A compiled byte based program for DFA execution. This is only used
    /// if a DFA can be executed. (Currently, only word boundary assertions are
    /// not supported.) Note that this program contains an embedded `.*?`
    /// preceding the first capture group, unless the regex is anchored at the
    /// beginning.
    dfa: Program,
    /// The same as above, except the program is reversed (and there is no
    /// preceding `.*?`). This is used by the DFA to find the starting location
    /// of matches.
    dfa_reverse: Program,
    /// A set of suffix literals extracted from the regex.
    ///
    /// Prefix literals are stored on the `Program`, since they are used inside
    /// the matching engines.
    suffixes: LiteralSearcher,
    /// match_type encodes as much upfront knowledge about how we're going to
    /// execute a search as possible.
    match_type: MatchType,
}
impl Exec {
    #[inline(always)]
    pub fn searcher(&self) -> ExecNoSync {}
    #[inline(always)]
    pub fn searcher_str(&self) -> ExecNoSyncStr {}
    pub fn into_regex(self) -> re_unicode::Regex {}
    pub fn into_regex_set(self) -> re_set::unicode::RegexSet {}
    pub fn into_byte_regex(self) -> re_bytes::Regex {}
    pub fn into_byte_regex_set(self) -> re_set::bytes::RegexSet {}
    pub fn regex_strings(&self) -> &[String] {}
    pub fn capture_names(&self) -> &[Option<String>] {}
    pub fn capture_name_idx(&self) -> &Arc<HashMap<String, usize>> {
        &self.ro.nfa.capture_name_idx
    }
}
