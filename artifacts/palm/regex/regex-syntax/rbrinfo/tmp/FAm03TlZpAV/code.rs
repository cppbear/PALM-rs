fn fmt_class_bracketed_pre(
        &mut self,
        ast: &ast::ClassBracketed,
    ) -> fmt::Result {
        if ast.negated {
            self.wtr.write_str("[^")
        } else {
            self.wtr.write_str("[")
        }
    }