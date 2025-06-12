fn parse_many(&self) -> Result<Vec<Hir>> {
        self.arg_patterns.iter().map(|s| parse(s)).collect()
    }