fn parse_any_number(&mut self, positive: bool) -> Result<ParserNumber> {
        self.parse_integer(positive)
    }