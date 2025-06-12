use std::fmt;
use ast::{self, Ast};
use ast::visitor::{self, Visitor};
pub trait Visitor {
    type Output;
    type Err;
    fn finish(self) -> Result<Self::Output, Self::Err>;
    fn start(&mut self);
    fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_class_set_item_pre(
        &mut self,
        _ast: &ast::ClassSetItem,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_class_set_item_post(
        &mut self,
        _ast: &ast::ClassSetItem,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_class_set_binary_op_pre(
        &mut self,
        _ast: &ast::ClassSetBinaryOp,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_class_set_binary_op_post(
        &mut self,
        _ast: &ast::ClassSetBinaryOp,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_class_set_binary_op_in(
        &mut self,
        _ast: &ast::ClassSetBinaryOp,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
}
#[derive(Debug)]
struct Writer<'p, W> {
    printer: &'p mut Printer,
    wtr: W,
}
#[derive(Debug)]
pub struct Printer {
    _priv: (),
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
#[derive(Debug)]
pub struct Printer {
    _priv: (),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassBracketed {
    /// The span of this class.
    pub span: Span,
    /// Whether this class is negated or not. e.g., `[a]` is not negated but
    /// `[^a]` is.
    pub negated: bool,
    /// The type of this set. A set is either a normal union of things, e.g.,
    /// `[abc]` or a result of applying set operations, e.g., `[\pL--c]`.
    pub kind: ClassSet,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
    /// The original pattern that the parser generated the error from. Every
    /// span in an error is a valid range into this string.
    pattern: String,
    /// The span of this error.
    span: Span,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// An error that occurred while translating concrete syntax into abstract
    /// syntax (AST).
    Parse(ast::Error),
    /// An error that occurred while translating abstract syntax into a high
    /// level intermediate representation (HIR).
    Translate(hir::Error),
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClassSetItem {
    /// An empty item.
    ///
    /// Note that a bracketed character class cannot contain a single empty
    /// item. Empty items can appear when using one of the binary operators.
    /// For example, `[&&]` is the intersection of two empty classes.
    Empty(Span),
    /// A single literal.
    Literal(Literal),
    /// A range between two literals.
    Range(ClassSetRange),
    /// An ASCII character class, e.g., `[:alnum:]` or `[:punct:]`.
    Ascii(ClassAscii),
    /// A Unicode character class, e.g., `\pL` or `\p{Greek}`.
    Unicode(ClassUnicode),
    /// A perl character class, e.g., `\d` or `\W`.
    Perl(ClassPerl),
    /// A bracketed character class set, which may contain zero or more
    /// character ranges and/or zero or more nested classes. e.g.,
    /// `[a-zA-Z\pL]`.
    Bracketed(Box<ClassBracketed>),
    /// A union of items.
    Union(ClassSetUnion),
}
#[derive(Debug)]
pub enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
}
impl<'p, W: fmt::Write> Visitor for Writer<'p, W> {
    type Output = ();
    type Err = fmt::Error;
    fn finish(self) -> fmt::Result {}
    fn visit_pre(&mut self, ast: &Ast) -> fmt::Result {}
    fn visit_post(&mut self, ast: &Ast) -> fmt::Result {}
    fn visit_alternation_in(&mut self) -> fmt::Result {}
    fn visit_class_set_item_pre(
        &mut self,
        ast: &ast::ClassSetItem,
    ) -> Result<(), Self::Err> {
        match *ast {
            ast::ClassSetItem::Bracketed(ref x) => self.fmt_class_bracketed_pre(x),
            _ => Ok(()),
        }
    }
    fn visit_class_set_item_post(
        &mut self,
        ast: &ast::ClassSetItem,
    ) -> Result<(), Self::Err> {
        use ast::ClassSetItem::*;
        match *ast {
            Empty(_) => Ok(()),
            Literal(ref x) => self.fmt_literal(x),
            Range(ref x) => {
                self.fmt_literal(&x.start)?;
                self.wtr.write_str("-")?;
                self.fmt_literal(&x.end)?;
                Ok(())
            }
            Ascii(ref x) => self.fmt_class_ascii(x),
            Unicode(ref x) => self.fmt_class_unicode(x),
            Perl(ref x) => self.fmt_class_perl(x),
            Bracketed(ref x) => self.fmt_class_bracketed_post(x),
            Union(_) => Ok(()),
        }
    }
    fn visit_class_set_binary_op_in(
        &mut self,
        ast: &ast::ClassSetBinaryOp,
    ) -> Result<(), Self::Err> {
        self.fmt_class_set_binary_op_kind(&ast.kind)
    }
}
impl<'p, W: fmt::Write> Writer<'p, W> {
    fn fmt_group_pre(&mut self, ast: &ast::Group) -> fmt::Result {}
    fn fmt_group_post(&mut self, _ast: &ast::Group) -> fmt::Result {}
    fn fmt_repetition(&mut self, ast: &ast::Repetition) -> fmt::Result {}
    fn fmt_repetition_range(&mut self, ast: &ast::RepetitionRange) -> fmt::Result {}
    fn fmt_literal(&mut self, ast: &ast::Literal) -> fmt::Result {}
    fn fmt_assertion(&mut self, ast: &ast::Assertion) -> fmt::Result {}
    fn fmt_set_flags(&mut self, ast: &ast::SetFlags) -> fmt::Result {}
    fn fmt_flags(&mut self, ast: &ast::Flags) -> fmt::Result {}
    fn fmt_class_bracketed_pre(&mut self, ast: &ast::ClassBracketed) -> fmt::Result {
        if ast.negated { self.wtr.write_str("[^") } else { self.wtr.write_str("[") }
    }
    fn fmt_class_bracketed_post(&mut self, _ast: &ast::ClassBracketed) -> fmt::Result {}
    fn fmt_class_set_binary_op_kind(
        &mut self,
        ast: &ast::ClassSetBinaryOpKind,
    ) -> fmt::Result {}
    fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {}
    fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result {}
    fn fmt_class_unicode(&mut self, ast: &ast::ClassUnicode) -> fmt::Result {}
}
