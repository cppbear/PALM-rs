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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaptureName {
    /// The span of this capture name.
    pub span: Span,
    /// The capture name.
    pub name: String,
    /// The capture index.
    pub index: u32,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Group {
    /// The kind of this group. If it is a capturing group, then the kind
    /// contains the capture group index (and the name, if it is a named
    /// group).
    pub kind: GroupKind,
    /// The expression inside the capturing group, which may be empty.
    pub hir: Box<Hir>,
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
pub struct Group {
    /// The span of this group.
    pub span: Span,
    /// The kind of this group.
    pub kind: GroupKind,
    /// The regular expression in this group.
    pub ast: Box<Ast>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GroupKind {
    /// A normal unnamed capturing group.
    ///
    /// The value is the capture index of the group.
    CaptureIndex(u32),
    /// A named capturing group.
    CaptureName {
        /// The name of the group.
        name: String,
        /// The capture index of the group.
        index: u32,
    },
    /// A non-capturing group.
    NonCapturing,
}
impl<'t, 'p> TranslatorI<'t, 'p> {
    fn new(trans: &'t Translator, pattern: &'p str) -> TranslatorI<'t, 'p> {}
    fn trans(&self) -> &Translator {}
    fn push(&self, frame: HirFrame) {}
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
    fn hir_group(&self, group: &ast::Group, expr: Hir) -> Hir {
        let kind = match group.kind {
            ast::GroupKind::CaptureIndex(idx) => hir::GroupKind::CaptureIndex(idx),
            ast::GroupKind::CaptureName(ref capname) => {
                hir::GroupKind::CaptureName {
                    name: capname.name.clone(),
                    index: capname.index,
                }
            }
            ast::GroupKind::NonCapturing(_) => hir::GroupKind::NonCapturing,
        };
        Hir::group(hir::Group {
            kind: kind,
            hir: Box::new(expr),
        })
    }
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
    pub fn literal(lit: Literal) -> Hir {}
    pub fn class(class: Class) -> Hir {}
    pub fn anchor(anchor: Anchor) -> Hir {}
    pub fn word_boundary(word_boundary: WordBoundary) -> Hir {}
    pub fn repetition(rep: Repetition) -> Hir {}
    pub fn group(group: Group) -> Hir {
        let mut info = HirInfo::new();
        info.set_always_utf8(group.hir.is_always_utf8());
        info.set_all_assertions(group.hir.is_all_assertions());
        info.set_anchored_start(group.hir.is_anchored_start());
        info.set_anchored_end(group.hir.is_anchored_end());
        info.set_any_anchored_start(group.hir.is_any_anchored_start());
        info.set_any_anchored_end(group.hir.is_any_anchored_end());
        info.set_match_empty(group.hir.is_match_empty());
        Hir {
            kind: HirKind::Group(group),
            info: info,
        }
    }
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
