pub fn parse(&mut self, pattern: &str) -> Result<Ast> {
        ParserI::new(self, pattern).parse()
    }