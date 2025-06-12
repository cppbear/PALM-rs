use std::fmt;
use hir::{self, Hir, HirKind};
use hir::visitor::{self, Visitor};
use is_meta_character;
pub trait Visitor {
    type Output;
    type Err;
    fn finish(self) -> Result<Self::Output, Self::Err>;
    fn start(&mut self);
    fn visit_pre(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_post(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
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
impl<'p, W: fmt::Write> Writer<'p, W> {
    fn write_literal_char(&mut self, c: char) -> fmt::Result {
        if is_meta_character(c) {
            self.wtr.write_str("\\")?;
        }
        self.wtr.write_char(c)
    }
    fn write_literal_byte(&mut self, b: u8) -> fmt::Result {}
    fn write_literal_class_byte(&mut self, b: u8) -> fmt::Result {}
}
pub fn is_meta_character(c: char) -> bool {
    match c {
        '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^'
        | '$' | '#' | '&' | '-' | '~' => true,
        _ => false,
    }
}
