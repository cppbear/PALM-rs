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
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
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
    fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {}
    fn hir_unicode_class(
        &self,
        ast_class: &ast::ClassUnicode,
    ) -> Result<hir::ClassUnicode> {}
    fn hir_perl_unicode_class(&self, ast_class: &ast::ClassPerl) -> hir::ClassUnicode {}
    fn hir_perl_byte_class(&self, ast_class: &ast::ClassPerl) -> hir::ClassBytes {}
    fn unicode_fold_and_negate(&self, negated: bool, class: &mut hir::ClassUnicode) {
        if self.flags().case_insensitive() {
            class.case_fold_simple();
        }
        if negated {
            class.negate();
        }
    }
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
    fn case_insensitive(&self) -> bool {
        self.case_insensitive.unwrap_or(false)
    }
    fn multi_line(&self) -> bool {}
    fn dot_matches_new_line(&self) -> bool {}
    fn swap_greed(&self) -> bool {}
    fn unicode(&self) -> bool {}
}
impl ClassUnicode {
    pub fn new<I>(ranges: I) -> ClassUnicode
    where
        I: IntoIterator<Item = ClassUnicodeRange>,
    {}
    pub fn empty() -> ClassUnicode {}
    pub fn push(&mut self, range: ClassUnicodeRange) {}
    pub fn iter(&self) -> ClassUnicodeIter {}
    pub fn ranges(&self) -> &[ClassUnicodeRange] {}
    pub fn case_fold_simple(&mut self) {
        self.set.case_fold_simple();
    }
    pub fn negate(&mut self) {
        self.set.negate();
    }
    pub fn union(&mut self, other: &ClassUnicode) {}
    pub fn intersect(&mut self, other: &ClassUnicode) {}
    pub fn difference(&mut self, other: &ClassUnicode) {}
    pub fn symmetric_difference(&mut self, other: &ClassUnicode) {}
}
