fn parse_octal(&self) -> ast::Literal {
        use std::char;
        use std::u32;

        assert!(self.parser().octal);
        assert!('0' <= self.char() && self.char() <= '7');
        let start = self.pos();
        // Parse up to two more digits.
        while
            self.bump() &&
            '0' <= self.char() && self.char() <= '7' &&
            self.pos().offset - start.offset <= 2
        {}
        let end = self.pos();
        let octal = &self.pattern()[start.offset..end.offset];
        // Parsing the octal should never fail since the above guarantees a
        // valid number.
        let codepoint =
            u32::from_str_radix(octal, 8).expect("valid octal number");
        // The max value for 3 digit octal is 0777 = 511 and [0, 511] has no
        // invalid Unicode scalar values.
        let c = char::from_u32(codepoint).expect("Unicode scalar value");
        ast::Literal {
            span: Span::new(start, end),
            kind: ast::LiteralKind::Octal,
            c: c,
        }
    }