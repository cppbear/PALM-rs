pub fn print<W: fmt::Write>(&mut self, hir: &Hir, wtr: W) -> fmt::Result {
        visitor::visit(hir, Writer { printer: self, wtr: wtr })
    }