use std::fmt;
use ast::{self, Ast};
use ast::visitor::{self, Visitor};
#[derive(Clone, Debug)]
struct PrinterBuilder {
    _priv: (),
}
impl PrinterBuilder {
    fn new() -> PrinterBuilder {
        PrinterBuilder { _priv: () }
    }
    fn build(&self) -> Printer {}
}
