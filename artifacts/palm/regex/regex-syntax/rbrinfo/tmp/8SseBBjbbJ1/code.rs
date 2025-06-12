fn parse_hex(&self) -> Result<ast::Literal> {
        assert!(self.char() == 'x'
                || self.char() == 'u'
                || self.char() == 'U');

        let hex_kind = match self.char() {
            'x' => ast::HexLiteralKind::X,
            'u' => ast::HexLiteralKind::UnicodeShort,
            _ => ast::HexLiteralKind::UnicodeLong,
        };
        if !self.bump_and_bump_space() {
            return Err(self.error(
                self.span(),
                ast::ErrorKind::EscapeUnexpectedEof,
            ));
        }
        if self.char() == '{' {
            self.parse_hex_brace(hex_kind)
        } else {
            self.parse_hex_digits(hex_kind)
        }
    }