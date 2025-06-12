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
#[derive(Clone, Debug)]
enum HirFrame {
    /// An arbitrary HIR expression. These get pushed whenever we hit a base
    /// case in the Ast. They get popped after an inductive (i.e., recursive)
    /// step is complete.
    Expr(Hir),
    /// A Unicode character class. This frame is mutated as we descend into
    /// the Ast of a character class (which is itself its own mini recursive
    /// structure).
    ClassUnicode(hir::ClassUnicode),
    /// A byte-oriented character class. This frame is mutated as we descend
    /// into the Ast of a character class (which is itself its own mini
    /// recursive structure).
    ///
    /// Byte character classes are created when Unicode mode (`u`) is disabled.
    /// If `allow_invalid_utf8` is disabled (the default), then a byte
    /// character is only permitted to match ASCII text.
    ClassBytes(hir::ClassBytes),
    /// This is pushed on to the stack upon first seeing any kind of group,
    /// indicated by parentheses (including non-capturing groups). It is popped
    /// upon leaving a group.
    Group {
        /// The old active flags, if any, when this group was opened.
        ///
        /// If this group sets flags, then the new active flags are set to the
        /// result of merging the old flags with the flags introduced by this
        /// group.
        ///
        /// When this group is popped, the active flags should be restored to
        /// the flags set here.
        ///
        /// The "active" flags correspond to whatever flags are set in the
        /// Translator.
        old_flags: Option<Flags>,
    },
    /// This is pushed whenever a concatenation is observed. After visiting
    /// every sub-expression in the concatenation, the translator's stack is
    /// popped until it sees a Concat frame.
    Concat,
    /// This is pushed whenever an alternation is observed. After visiting
    /// every sub-expression in the alternation, the translator's stack is
    /// popped until it sees an Alternation frame.
    Alternation,
}
impl<'t, 'p> TranslatorI<'t, 'p> {
    fn new(trans: &'t Translator, pattern: &'p str) -> TranslatorI<'t, 'p> {}
    fn trans(&self) -> &Translator {
        &self.trans
    }
    fn push(&self, frame: HirFrame) {
        self.trans().stack.borrow_mut().push(frame);
    }
    fn pop(&self) -> Option<HirFrame> {}
    fn error(&self, span: Span, kind: ErrorKind) -> Error {}
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
