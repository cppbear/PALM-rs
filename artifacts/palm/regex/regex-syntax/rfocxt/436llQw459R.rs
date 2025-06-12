use std::fmt;
use ast::{self, Ast};
use ast::visitor::{self, Visitor};
#[derive(Clone, Debug)]
struct PrinterBuilder {
    _priv: (),
}
#[derive(Debug)]
pub struct Printer {
    _priv: (),
}
impl PrinterBuilder {
    fn new() -> PrinterBuilder {}
    fn build(&self) -> Printer {
        Printer { _priv: () }
    }
}
