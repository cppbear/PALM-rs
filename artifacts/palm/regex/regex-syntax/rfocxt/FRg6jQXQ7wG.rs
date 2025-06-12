use std::fmt;
use hir::{self, Hir, HirKind};
use hir::visitor::{self, Visitor};
use is_meta_character;
#[derive(Debug)]
pub struct Printer {
    _priv: (),
}
#[derive(Debug)]
struct Writer<'p, W> {
    printer: &'p mut Printer,
    wtr: W,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    info: HirInfo,
}
impl Printer {
    pub fn new() -> Printer {}
    pub fn print<W: fmt::Write>(&mut self, hir: &Hir, wtr: W) -> fmt::Result {
        visitor::visit(hir, Writer { printer: self, wtr: wtr })
    }
}
