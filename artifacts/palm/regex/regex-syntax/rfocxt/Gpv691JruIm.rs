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
#[derive(Debug)]
pub struct Printer {
    _priv: (),
}
#[derive(Debug)]
pub enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
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
impl<'p, W: fmt::Write> Visitor for Writer<'p, W> {
    type Output = ();
    type Err = fmt::Error;
    fn finish(self) -> fmt::Result {
        Ok(())
    }
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
