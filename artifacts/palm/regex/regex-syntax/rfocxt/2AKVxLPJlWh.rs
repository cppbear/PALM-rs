use std::fmt;
use hir::{self, Hir, HirKind};
use hir::visitor::{self, Visitor};
use is_meta_character;
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
