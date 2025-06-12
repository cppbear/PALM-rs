fn parse_one(&self) -> Result<Hir> {
        parse(&self.arg_pattern)
    }