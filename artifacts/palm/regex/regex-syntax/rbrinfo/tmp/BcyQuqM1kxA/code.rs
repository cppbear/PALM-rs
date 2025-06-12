fn parse_flag(&self) -> Result<ast::Flag> {
        match self.char() {
            'i' => Ok(ast::Flag::CaseInsensitive),
            'm' => Ok(ast::Flag::MultiLine),
            's' => Ok(ast::Flag::DotMatchesNewLine),
            'U' => Ok(ast::Flag::SwapGreed),
            'u' => Ok(ast::Flag::Unicode),
            'x' => Ok(ast::Flag::IgnoreWhitespace),
            _ => Err(self.error(
                self.span_char(),
                ast::ErrorKind::FlagUnrecognized,
            )),
        }
    }