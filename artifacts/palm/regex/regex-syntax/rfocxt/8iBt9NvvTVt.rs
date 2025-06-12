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
pub struct Literal {
    /// The span of this literal.
    pub span: Span,
    /// The kind of this literal.
    pub kind: LiteralKind,
    /// The Unicode scalar value corresponding to this literal.
    pub c: char,
}
#[derive(Debug)]
pub struct Printer {
    _priv: (),
}
impl<'p, W: fmt::Write> Writer<'p, W> {
    fn fmt_group_pre(&mut self, ast: &ast::Group) -> fmt::Result {}
    fn fmt_group_post(&mut self, _ast: &ast::Group) -> fmt::Result {}
    fn fmt_repetition(&mut self, ast: &ast::Repetition) -> fmt::Result {}
    fn fmt_repetition_range(&mut self, ast: &ast::RepetitionRange) -> fmt::Result {}
    fn fmt_literal(&mut self, ast: &ast::Literal) -> fmt::Result {
        use ast::LiteralKind::*;
        match ast.kind {
            Verbatim => self.wtr.write_char(ast.c),
            Punctuation => write!(self.wtr, r"\{}", ast.c),
            Octal => write!(self.wtr, r"\{:o}", ast.c as u32),
            HexFixed(ast::HexLiteralKind::X) => {
                write!(self.wtr, r"\x{:02X}", ast.c as u32)
            }
            HexFixed(ast::HexLiteralKind::UnicodeShort) => {
                write!(self.wtr, r"\u{:04X}", ast.c as u32)
            }
            HexFixed(ast::HexLiteralKind::UnicodeLong) => {
                write!(self.wtr, r"\U{:08X}", ast.c as u32)
            }
            HexBrace(ast::HexLiteralKind::X) => {
                write!(self.wtr, r"\x{{{:X}}}", ast.c as u32)
            }
            HexBrace(ast::HexLiteralKind::UnicodeShort) => {
                write!(self.wtr, r"\u{{{:X}}}", ast.c as u32)
            }
            HexBrace(ast::HexLiteralKind::UnicodeLong) => {
                write!(self.wtr, r"\U{{{:X}}}", ast.c as u32)
            }
            Special(ast::SpecialLiteralKind::Bell) => self.wtr.write_str(r"\a"),
            Special(ast::SpecialLiteralKind::FormFeed) => self.wtr.write_str(r"\f"),
            Special(ast::SpecialLiteralKind::Tab) => self.wtr.write_str(r"\t"),
            Special(ast::SpecialLiteralKind::LineFeed) => self.wtr.write_str(r"\n"),
            Special(ast::SpecialLiteralKind::CarriageReturn) => self.wtr.write_str(r"\r"),
            Special(ast::SpecialLiteralKind::VerticalTab) => self.wtr.write_str(r"\v"),
            Special(ast::SpecialLiteralKind::Space) => self.wtr.write_str(r"\ "),
        }
    }
    fn fmt_assertion(&mut self, ast: &ast::Assertion) -> fmt::Result {}
    fn fmt_set_flags(&mut self, ast: &ast::SetFlags) -> fmt::Result {}
    fn fmt_flags(&mut self, ast: &ast::Flags) -> fmt::Result {}
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
