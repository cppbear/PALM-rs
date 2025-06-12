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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassAscii {
    /// The span of this class.
    pub span: Span,
    /// The kind of ASCII class.
    pub kind: ClassAsciiKind,
    /// Whether the class is negated or not. e.g., `[[:alpha:]]` is not negated
    /// but `[[:^alpha:]]` is.
    pub negated: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClassAsciiKind {
    /// `[0-9A-Za-z]`
    Alnum,
    /// `[A-Za-z]`
    Alpha,
    /// `[\x00-\x7F]`
    Ascii,
    /// `[ \t]`
    Blank,
    /// `[\x00-\x1F\x7F]`
    Cntrl,
    /// `[0-9]`
    Digit,
    /// `[!-~]`
    Graph,
    /// `[a-z]`
    Lower,
    /// `[ -~]`
    Print,
    /// `[!-/:-@\[-`{-~]`
    Punct,
    /// `[\t\n\v\f\r ]`
    Space,
    /// `[A-Z]`
    Upper,
    /// `[0-9A-Za-z_]`
    Word,
    /// `[0-9A-Fa-f]`
    Xdigit,
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
    ) -> fmt::Result {}
    fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {}
    fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result {
        use ast::ClassAsciiKind::*;
        match ast.kind {
            Alnum if ast.negated => self.wtr.write_str("[:^alnum:]"),
            Alnum => self.wtr.write_str("[:alnum:]"),
            Alpha if ast.negated => self.wtr.write_str("[:^alpha:]"),
            Alpha => self.wtr.write_str("[:alpha:]"),
            Ascii if ast.negated => self.wtr.write_str("[:^ascii:]"),
            Ascii => self.wtr.write_str("[:ascii:]"),
            Blank if ast.negated => self.wtr.write_str("[:^blank:]"),
            Blank => self.wtr.write_str("[:blank:]"),
            Cntrl if ast.negated => self.wtr.write_str("[:^cntrl:]"),
            Cntrl => self.wtr.write_str("[:cntrl:]"),
            Digit if ast.negated => self.wtr.write_str("[:^digit:]"),
            Digit => self.wtr.write_str("[:digit:]"),
            Graph if ast.negated => self.wtr.write_str("[:^graph:]"),
            Graph => self.wtr.write_str("[:graph:]"),
            Lower if ast.negated => self.wtr.write_str("[:^lower:]"),
            Lower => self.wtr.write_str("[:lower:]"),
            Print if ast.negated => self.wtr.write_str("[:^print:]"),
            Print => self.wtr.write_str("[:print:]"),
            Punct if ast.negated => self.wtr.write_str("[:^punct:]"),
            Punct => self.wtr.write_str("[:punct:]"),
            Space if ast.negated => self.wtr.write_str("[:^space:]"),
            Space => self.wtr.write_str("[:space:]"),
            Upper if ast.negated => self.wtr.write_str("[:^upper:]"),
            Upper => self.wtr.write_str("[:upper:]"),
            Word if ast.negated => self.wtr.write_str("[:^word:]"),
            Word => self.wtr.write_str("[:word:]"),
            Xdigit if ast.negated => self.wtr.write_str("[:^xdigit:]"),
            Xdigit => self.wtr.write_str("[:xdigit:]"),
        }
    }
    fn fmt_class_unicode(&mut self, ast: &ast::ClassUnicode) -> fmt::Result {}
}
