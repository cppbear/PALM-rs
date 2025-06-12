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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    info: HirInfo,
}
#[derive(Clone, Copy, Debug, Default)]
struct Flags {
    case_insensitive: Option<bool>,
    multi_line: Option<bool>,
    dot_matches_new_line: Option<bool>,
    swap_greed: Option<bool>,
    unicode: Option<bool>,
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
pub struct Literal {
    /// The span of this literal.
    pub span: Span,
    /// The kind of this literal.
    pub kind: LiteralKind,
    /// The Unicode scalar value corresponding to this literal.
    pub c: char,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Span {
    /// The start byte offset.
    pub start: Position,
    /// The end byte offset.
    pub end: Position,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal {
    /// A single character represented by a Unicode scalar value.
    Unicode(char),
    /// A single character represented by an arbitrary byte.
    Byte(u8),
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
    fn hir_literal(&self, lit: &ast::Literal) -> Result<Hir> {
        let ch = match self.literal_to_char(lit)? {
            byte @ hir::Literal::Byte(_) => return Ok(Hir::literal(byte)),
            hir::Literal::Unicode(ch) => ch,
        };
        if self.flags().case_insensitive() {
            self.hir_from_char_case_insensitive(lit.span, ch)
        } else {
            self.hir_from_char(lit.span, ch)
        }
    }
    fn literal_to_char(&self, lit: &ast::Literal) -> Result<hir::Literal> {
        if self.flags().unicode() {
            return Ok(hir::Literal::Unicode(lit.c));
        }
        let byte = match lit.byte() {
            None => return Ok(hir::Literal::Unicode(lit.c)),
            Some(byte) => byte,
        };
        if byte <= 0x7F {
            return Ok(hir::Literal::Unicode(byte as char));
        }
        if !self.trans().allow_invalid_utf8 {
            return Err(self.error(lit.span, ErrorKind::InvalidUtf8));
        }
        Ok(hir::Literal::Byte(byte))
    }
    fn hir_from_char(&self, span: Span, c: char) -> Result<Hir> {
        if !self.flags().unicode() && c.len_utf8() > 1 {
            return Err(self.error(span, ErrorKind::UnicodeNotAllowed));
        }
        Ok(Hir::literal(hir::Literal::Unicode(c)))
    }
    fn hir_from_char_case_insensitive(&self, span: Span, c: char) -> Result<Hir> {
        if !unicode::contains_simple_case_mapping(c, c) {
            return self.hir_from_char(span, c);
        }
        if self.flags().unicode() {
            let mut cls = hir::ClassUnicode::new(
                vec![hir::ClassUnicodeRange::new(c, c),],
            );
            cls.case_fold_simple();
            Ok(Hir::class(hir::Class::Unicode(cls)))
        } else {
            if c.len_utf8() > 1 {
                return Err(self.error(span, ErrorKind::UnicodeNotAllowed));
            }
            let mut cls = hir::ClassBytes::new(
                vec![hir::ClassBytesRange::new(c as u8, c as u8),],
            );
            cls.case_fold_simple();
            Ok(Hir::class(hir::Class::Bytes(cls)))
        }
    }
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
impl Hir {
    pub fn kind(&self) -> &HirKind {}
    pub fn into_kind(mut self) -> HirKind {}
    pub fn empty() -> Hir {}
    pub fn literal(lit: Literal) -> Hir {
        if let Literal::Byte(b) = lit {
            assert!(b > 0x7F);
        }
        let mut info = HirInfo::new();
        info.set_always_utf8(lit.is_unicode());
        info.set_all_assertions(false);
        info.set_anchored_start(false);
        info.set_anchored_end(false);
        info.set_any_anchored_start(false);
        info.set_any_anchored_end(false);
        info.set_match_empty(false);
        Hir {
            kind: HirKind::Literal(lit),
            info: info,
        }
    }
    pub fn class(class: Class) -> Hir {}
    pub fn anchor(anchor: Anchor) -> Hir {}
    pub fn word_boundary(word_boundary: WordBoundary) -> Hir {}
    pub fn repetition(rep: Repetition) -> Hir {}
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
