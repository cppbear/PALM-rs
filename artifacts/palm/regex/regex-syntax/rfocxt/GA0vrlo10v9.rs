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
#[derive(Debug)]
pub struct Printer {
    _priv: (),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassSetBinaryOpKind {
    /// The intersection of two sets, e.g., `\pN&&[a-z]`.
    Intersection,
    /// The difference of two sets, e.g., `\pN--[0-9]`.
    Difference,
    /// The symmetric difference of two sets. The symmetric difference is the
    /// set of elements belonging to one but not both sets.
    /// e.g., `[\pL~~[:ascii:]]`.
    SymmetricDifference,
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
    fn fmt_class_bracketed_pre(&mut self, ast: &ast::ClassBracketed) -> fmt::Result {}
    fn fmt_class_bracketed_post(&mut self, _ast: &ast::ClassBracketed) -> fmt::Result {}
    fn fmt_class_set_binary_op_kind(
        &mut self,
        ast: &ast::ClassSetBinaryOpKind,
    ) -> fmt::Result {
        use ast::ClassSetBinaryOpKind::*;
        match *ast {
            Intersection => self.wtr.write_str("&&"),
            Difference => self.wtr.write_str("--"),
            SymmetricDifference => self.wtr.write_str("~~"),
        }
    }
    fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {}
    fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result {}
    fn fmt_class_unicode(&mut self, ast: &ast::ClassUnicode) -> fmt::Result {}
}
