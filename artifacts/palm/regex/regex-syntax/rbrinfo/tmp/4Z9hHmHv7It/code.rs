fn fmt_group_post(&mut self, _ast: &ast::Group) -> fmt::Result {
        self.wtr.write_str(")")
    }