use std::fmt;
use hir::{self, Hir, HirKind};
use hir::visitor::{self, Visitor};
use is_meta_character;
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
