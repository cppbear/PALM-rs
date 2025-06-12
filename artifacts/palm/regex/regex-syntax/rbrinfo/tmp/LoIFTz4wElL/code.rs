fn fmt_class_bracketed_post(
        &mut self,
        _ast: &ast::ClassBracketed,
    ) -> fmt::Result {
        self.wtr.write_str("]")
    }