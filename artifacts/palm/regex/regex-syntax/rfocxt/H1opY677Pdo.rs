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
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
    /// The original pattern that the translator's Ast was parsed from. Every
    /// span in an error is a valid range into this string.
    pattern: String,
    /// The span of this error, derived from the Ast given to the translator.
    span: Span,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Span {
    /// The start byte offset.
    pub start: Position,
    /// The end byte offset.
    pub end: Position,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// This error occurs when a Unicode feature is used when Unicode
    /// support is disabled. For example `(?-u:\pL)` would trigger this error.
    UnicodeNotAllowed,
    /// This error occurs when translating a pattern that could match a byte
    /// sequence that isn't UTF-8 and `allow_invalid_utf8` was disabled.
    InvalidUtf8,
    /// This occurs when an unrecognized Unicode property name could not
    /// be found.
    UnicodePropertyNotFound,
    /// This occurs when an unrecognized Unicode property value could not
    /// be found.
    UnicodePropertyValueNotFound,
    /// This occurs when the translator attempts to construct a character class
    /// that is empty.
    ///
    /// Note that this restriction in the translator may be removed in the
    /// future.
    EmptyClassNotAllowed,
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}
impl<'t, 'p> TranslatorI<'t, 'p> {
    fn new(trans: &'t Translator, pattern: &'p str) -> TranslatorI<'t, 'p> {}
    fn trans(&self) -> &Translator {}
    fn push(&self, frame: HirFrame) {}
    fn pop(&self) -> Option<HirFrame> {}
    fn error(&self, span: Span, kind: ErrorKind) -> Error {
        Error {
            kind: kind,
            pattern: self.pattern.to_string(),
            span: span,
        }
    }
    fn flags(&self) -> Flags {}
    fn set_flags(&self, ast_flags: &ast::Flags) -> Flags {}
    fn hir_literal(&self, lit: &ast::Literal) -> Result<Hir> {}
    fn literal_to_char(&self, lit: &ast::Literal) -> Result<hir::Literal> {}
    fn hir_from_char(&self, span: Span, c: char) -> Result<Hir> {}
    fn hir_from_char_case_insensitive(&self, span: Span, c: char) -> Result<Hir> {}
    fn hir_dot(&self, span: Span) -> Result<Hir> {}
    fn hir_assertion(&self, asst: &ast::Assertion) -> Result<Hir> {}
    fn hir_group(&self, group: &ast::Group, expr: Hir) -> Hir {}
    fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {}
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
