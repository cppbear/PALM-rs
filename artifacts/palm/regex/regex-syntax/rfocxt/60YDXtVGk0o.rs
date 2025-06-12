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
    fn write_literal_char(&mut self, c: char) -> fmt::Result {}
    fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
        let c = b as char;
        if c <= 0x7F as char && !c.is_control() && !c.is_whitespace() {
            self.wtr.write_char(c)
        } else {
            write!(self.wtr, "(?-u:\\x{:02X})", b)
        }
    }
    fn write_literal_class_byte(&mut self, b: u8) -> fmt::Result {}
}
