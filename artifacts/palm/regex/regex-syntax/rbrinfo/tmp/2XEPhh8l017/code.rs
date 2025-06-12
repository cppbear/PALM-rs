fn parse_hex_digits(
        &self,
        kind: ast::HexLiteralKind,
    ) -> Result<ast::Literal> {
        use std::char;
        use std::u32;

        let mut scratch = self.parser().scratch.borrow_mut();
        scratch.clear();

        let start = self.pos();
        for i in 0..kind.digits() {
            if i > 0 && !self.bump_and_bump_space() {
                return Err(self.error(
                    self.span(),
                    ast::ErrorKind::EscapeUnexpectedEof,
                ));
            }
            if !is_hex(self.char()) {
                return Err(self.error(
                    self.span_char(),
                    ast::ErrorKind::EscapeHexInvalidDigit,
                ));
            }
            scratch.push(self.char());
        }
        // The final bump just moves the parser past the literal, which may
        // be EOF.
        self.bump_and_bump_space();
        let end = self.pos();
        let hex = scratch.as_str();
        match u32::from_str_radix(hex, 16).ok().and_then(char::from_u32) {
            None => Err(self.error(
                Span::new(start, end),
                ast::ErrorKind::EscapeHexInvalid,
            )),
            Some(c) => Ok(ast::Literal {
                span: Span::new(start, end),
                kind: ast::LiteralKind::HexFixed(kind),
                c: c,
            }),
        }
    }