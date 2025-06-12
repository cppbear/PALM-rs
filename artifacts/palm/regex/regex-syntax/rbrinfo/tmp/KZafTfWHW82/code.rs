fn parse(&self) -> Result<Ast> {
        self.parse_with_comments().map(|astc| astc.ast)
    }