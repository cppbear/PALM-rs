pub fn parse_with_comments(
        &mut self,
        pattern: &str,
    ) -> Result<ast::WithComments> {
        ParserI::new(self, pattern).parse_with_comments()
    }