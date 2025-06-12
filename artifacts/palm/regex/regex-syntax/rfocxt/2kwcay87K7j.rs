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
pub struct Flags {
    /// The span of this group of flags.
    pub span: Span,
    /// A sequence of flag items. Each item is either a flag or a negation
    /// operator.
    pub items: Vec<FlagsItem>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlagsItem {
    /// The span of this item.
    pub span: Span,
    /// The kind of this item.
    pub kind: FlagsItemKind,
}
#[derive(Debug)]
pub struct Printer {
    _priv: (),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Flag {
    /// `i`
    CaseInsensitive,
    /// `m`
    MultiLine,
    /// `s`
    DotMatchesNewLine,
    /// `U`
    SwapGreed,
    /// `u`
    Unicode,
    /// `x`
    IgnoreWhitespace,
}
impl<'p, W: fmt::Write> Writer<'p, W> {
    fn fmt_group_pre(&mut self, ast: &ast::Group) -> fmt::Result {}
    fn fmt_group_post(&mut self, _ast: &ast::Group) -> fmt::Result {}
    fn fmt_repetition(&mut self, ast: &ast::Repetition) -> fmt::Result {}
    fn fmt_repetition_range(&mut self, ast: &ast::RepetitionRange) -> fmt::Result {}
    fn fmt_literal(&mut self, ast: &ast::Literal) -> fmt::Result {}
    fn fmt_assertion(&mut self, ast: &ast::Assertion) -> fmt::Result {}
    fn fmt_set_flags(&mut self, ast: &ast::SetFlags) -> fmt::Result {}
    fn fmt_flags(&mut self, ast: &ast::Flags) -> fmt::Result {
        use ast::{Flag, FlagsItemKind};
        for item in &ast.items {
            match item.kind {
                FlagsItemKind::Negation => self.wtr.write_str("-"),
                FlagsItemKind::Flag(ref flag) => {
                    match *flag {
                        Flag::CaseInsensitive => self.wtr.write_str("i"),
                        Flag::MultiLine => self.wtr.write_str("m"),
                        Flag::DotMatchesNewLine => self.wtr.write_str("s"),
                        Flag::SwapGreed => self.wtr.write_str("U"),
                        Flag::Unicode => self.wtr.write_str("u"),
                        Flag::IgnoreWhitespace => self.wtr.write_str("x"),
                    }
                }
            }?;
        }
        Ok(())
    }
    fn fmt_class_bracketed_pre(&mut self, ast: &ast::ClassBracketed) -> fmt::Result {}
    fn fmt_class_bracketed_post(&mut self, _ast: &ast::ClassBracketed) -> fmt::Result {}
    fn fmt_class_set_binary_op_kind(
        &mut self,
        ast: &ast::ClassSetBinaryOpKind,
    ) -> fmt::Result {}
    fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {}
    fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result {}
    fn fmt_class_unicode(&mut self, ast: &ast::ClassUnicode) -> fmt::Result {}
}
