pub fn nest_limit(&mut self, limit: u32) -> &mut ParserBuilder {
        self.ast.nest_limit(limit);
        self
    }