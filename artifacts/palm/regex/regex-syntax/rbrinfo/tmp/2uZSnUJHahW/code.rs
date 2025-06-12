fn parse_hex_brace(
        &self,
        kind: ast::HexLiteralKind,
    ) -> Result<ast::Literal> {
        use std::char;
        use std::u32;

        let mut scratch = self.parser().scratch.borrow_mut();
        scratch.clear();

        let brace_pos = self.pos();
        let start = self.span_char().end;
        while self.bump_and_bump_space() && self.char() != '}' {
            if !is_hex(self.char()) {
                return Err(self.error(
                    self.span_char(),
                    ast::ErrorKind::EscapeHexInvalidDigit,
                ));
            }
            scratch.push(self.char());
        }
        if self.is_eof() {
            return Err(self.error(
                Span::new(brace_pos, self.pos()),
                ast::ErrorKind::EscapeUnexpectedEof,
            ));
        }
        let end = self.pos();
        let hex = scratch.as_str();
        assert_eq!(self.char(), '}');
        self.bump_and_bump_space();

        if hex.is_empty() {
            return Err(self.error(
                Span::new(brace_pos, self.pos()),
                ast::ErrorKind::EscapeHexEmpty,
            ));
        }
        match u32::from_str_radix(hex, 16).ok().and_then(char::from_u32) {
            None => Err(self.error(
                Span::new(start, end),
                ast::ErrorKind::EscapeHexInvalid,
            )),
            Some(c) => Ok(ast::Literal {
                span: Span::new(start, self.pos()),
                kind: ast::LiteralKind::HexBrace(kind),
                c: c,
            }),
        }
    }