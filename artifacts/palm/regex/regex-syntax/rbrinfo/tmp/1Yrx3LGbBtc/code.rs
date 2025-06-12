fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {
        use ast::ClassPerlKind::*;
        match ast.kind {
            Digit if ast.negated => self.wtr.write_str(r"\D"),
            Digit => self.wtr.write_str(r"\d"),
            Space if ast.negated => self.wtr.write_str(r"\S"),
            Space => self.wtr.write_str(r"\s"),
            Word if ast.negated => self.wtr.write_str(r"\W"),
            Word => self.wtr.write_str(r"\w"),
        }
    }