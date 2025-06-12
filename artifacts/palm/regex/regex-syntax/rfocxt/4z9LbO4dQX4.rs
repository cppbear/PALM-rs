type Result<T> = result::Result<T, Error>;
use std::cell::{Cell, RefCell};
use std::result;
use ast::{self, Ast, Span, Visitor};
use hir::{self, Error, ErrorKind, Hir};
use unicode::{self, ClassQuery};
#[derive(Clone, Debug)]
struct TranslatorI<'t, 'p> {
    trans: &'t Translator,
    pattern: &'p str,
}
#[derive(Clone, Copy, Debug, Default)]
struct Flags {
    case_insensitive: Option<bool>,
    multi_line: Option<bool>,
    dot_matches_new_line: Option<bool>,
    swap_greed: Option<bool>,
    unicode: Option<bool>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    info: HirInfo,
}
#[derive(Clone, Debug)]
pub struct Translator {
    /// Our call stack, but on the heap.
    stack: RefCell<Vec<HirFrame>>,
    /// The current flag settings.
    flags: Cell<Flags>,
    /// Whether we're allowed to produce HIR that can match arbitrary bytes.
    allow_invalid_utf8: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Repetition {
    /// The kind of this repetition operator.
    pub kind: RepetitionKind,
    /// Whether this repetition operator is greedy or not. A greedy operator
    /// will match as much as it can. A non-greedy operator will match as
    /// little as it can.
    ///
    /// Typically, operators are greedy by default and are only non-greedy when
    /// a `?` suffix is used, e.g., `(expr)*` is greedy while `(expr)*?` is
    /// not. However, this can be inverted via the `U` "ungreedy" flag.
    pub greedy: bool,
    /// The expression being repeated.
    pub hir: Box<Hir>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Repetition {
    /// The span of this operation.
    pub span: Span,
    /// The actual operation.
    pub op: RepetitionOp,
    /// Whether this operation was applied greedily or not.
    pub greedy: bool,
    /// The regular expression under repetition.
    pub ast: Box<Ast>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepetitionRange {
    /// Matches a sub-expression exactly this many times.
    Exactly(u32),
    /// Matches a sub-expression at least this many times.
    AtLeast(u32),
    /// Matches a sub-expression at least `m` times and at most `n` times.
    Bounded(u32, u32),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepetitionKind {
    /// Matches a sub-expression zero or one times.
    ZeroOrOne,
    /// Matches a sub-expression zero or more times.
    ZeroOrMore,
    /// Matches a sub-expression one or more times.
    OneOrMore,
    /// Matches a sub-expression within a bounded range of times.
    Range(RepetitionRange),
}
impl<'t, 'p> TranslatorI<'t, 'p> {
    fn new(trans: &'t Translator, pattern: &'p str) -> TranslatorI<'t, 'p> {}
    fn trans(&self) -> &Translator {}
    fn push(&self, frame: HirFrame) {}
    fn pop(&self) -> Option<HirFrame> {}
    fn error(&self, span: Span, kind: ErrorKind) -> Error {}
    fn flags(&self) -> Flags {
        self.trans().flags.get()
    }
    fn set_flags(&self, ast_flags: &ast::Flags) -> Flags {}
    fn hir_literal(&self, lit: &ast::Literal) -> Result<Hir> {}
    fn literal_to_char(&self, lit: &ast::Literal) -> Result<hir::Literal> {}
    fn hir_from_char(&self, span: Span, c: char) -> Result<Hir> {}
    fn hir_from_char_case_insensitive(&self, span: Span, c: char) -> Result<Hir> {}
    fn hir_dot(&self, span: Span) -> Result<Hir> {}
    fn hir_assertion(&self, asst: &ast::Assertion) -> Result<Hir> {}
    fn hir_group(&self, group: &ast::Group, expr: Hir) -> Hir {}
    fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {
        let kind = match rep.op.kind {
            ast::RepetitionKind::ZeroOrOne => hir::RepetitionKind::ZeroOrOne,
            ast::RepetitionKind::ZeroOrMore => hir::RepetitionKind::ZeroOrMore,
            ast::RepetitionKind::OneOrMore => hir::RepetitionKind::OneOrMore,
            ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(m)) => {
                hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(m))
            }
            ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(m)) => {
                hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(m))
            }
            ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(m, n)) => {
                hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(m, n))
            }
        };
        let greedy = if self.flags().swap_greed() { !rep.greedy } else { rep.greedy };
        Hir::repetition(hir::Repetition {
            kind: kind,
            greedy: greedy,
            hir: Box::new(expr),
        })
    }
    fn hir_unicode_class(
        &self,
        ast_class: &ast::ClassUnicode,
    ) -> Result<hir::ClassUnicode> {}
    fn hir_perl_unicode_class(&self, ast_class: &ast::ClassPerl) -> hir::ClassUnicode {}
    fn hir_perl_byte_class(&self, ast_class: &ast::ClassPerl) -> hir::ClassBytes {}
    fn unicode_fold_and_negate(&self, negated: bool, class: &mut hir::ClassUnicode) {}
    fn bytes_fold_and_negate(
        &self,
        span: &Span,
        negated: bool,
        class: &mut hir::ClassBytes,
    ) -> Result<()> {}
    fn class_literal_byte(&self, ast: &ast::Literal) -> Result<u8> {}
}
impl Flags {
    fn from_ast(ast: &ast::Flags) -> Flags {}
    fn merge(&mut self, previous: &Flags) {}
    fn case_insensitive(&self) -> bool {}
    fn multi_line(&self) -> bool {}
    fn dot_matches_new_line(&self) -> bool {}
    fn swap_greed(&self) -> bool {
        self.swap_greed.unwrap_or(false)
    }
    fn unicode(&self) -> bool {}
}
impl Hir {
    pub fn kind(&self) -> &HirKind {}
    pub fn into_kind(mut self) -> HirKind {}
    pub fn empty() -> Hir {}
    pub fn literal(lit: Literal) -> Hir {}
    pub fn class(class: Class) -> Hir {}
    pub fn anchor(anchor: Anchor) -> Hir {}
    pub fn word_boundary(word_boundary: WordBoundary) -> Hir {}
    pub fn repetition(rep: Repetition) -> Hir {
        let mut info = HirInfo::new();
        info.set_always_utf8(rep.hir.is_always_utf8());
        info.set_all_assertions(rep.hir.is_all_assertions());
        info.set_anchored_start(!rep.is_match_empty() && rep.hir.is_anchored_start());
        info.set_anchored_end(!rep.is_match_empty() && rep.hir.is_anchored_end());
        info.set_any_anchored_start(rep.hir.is_any_anchored_start());
        info.set_any_anchored_end(rep.hir.is_any_anchored_end());
        info.set_match_empty(rep.is_match_empty() || rep.hir.is_match_empty());
        Hir {
            kind: HirKind::Repetition(rep),
            info: info,
        }
    }
    pub fn group(group: Group) -> Hir {}
    pub fn concat(mut exprs: Vec<Hir>) -> Hir {}
    pub fn alternation(mut exprs: Vec<Hir>) -> Hir {}
    pub fn dot(bytes: bool) -> Hir {}
    pub fn any(bytes: bool) -> Hir {}
    pub fn is_always_utf8(&self) -> bool {}
    pub fn is_all_assertions(&self) -> bool {}
    pub fn is_anchored_start(&self) -> bool {}
    pub fn is_anchored_end(&self) -> bool {}
    pub fn is_any_anchored_start(&self) -> bool {}
    pub fn is_any_anchored_end(&self) -> bool {}
    pub fn is_match_empty(&self) -> bool {}
}
