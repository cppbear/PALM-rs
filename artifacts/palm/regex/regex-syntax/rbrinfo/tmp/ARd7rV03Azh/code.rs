pub fn print<W: fmt::Write>(&mut self, ast: &Ast, wtr: W) -> fmt::Result {
        visitor::visit(ast, Writer { printer: self, wtr: wtr })
    }