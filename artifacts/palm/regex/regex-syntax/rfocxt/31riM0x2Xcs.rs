type Result<T> = result::Result<T, Error>;
use std::cell::{Cell, RefCell};
use std::result;
use ast::{self, Ast, Span, Visitor};
use hir::{self, Error, ErrorKind, Hir};
use unicode::{self, ClassQuery};
#[derive(Clone, Debug)]
pub struct TranslatorBuilder {
    allow_invalid_utf8: bool,
    flags: Flags,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Flags {
    /// The span of this group of flags.
    pub span: Span,
    /// A sequence of flag items. Each item is either a flag or a negation
    /// operator.
    pub items: Vec<FlagsItem>,
}
#[derive(Clone, Copy, Debug, Default)]
struct Flags {
    case_insensitive: Option<bool>,
    multi_line: Option<bool>,
    dot_matches_new_line: Option<bool>,
    swap_greed: Option<bool>,
    unicode: Option<bool>,
}
impl TranslatorBuilder {
    pub fn new() -> TranslatorBuilder {}
    pub fn build(&self) -> Translator {}
    pub fn allow_invalid_utf8(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn unicode(&mut self, yes: bool) -> &mut TranslatorBuilder {
        self.flags.unicode = if yes { None } else { Some(false) };
        self
    }
}
