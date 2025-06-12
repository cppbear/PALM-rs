fn parse_escape(&self) -> Result<Primitive> {
        assert_eq!(self.char(), '\\');
        let start = self.pos();
        if !self.bump() {
            return Err(self.error(
                Span::new(start, self.pos()),
                ast::ErrorKind::EscapeUnexpectedEof,
            ));
        }
        let c = self.char();
        // Put some of the more complicated routines into helpers.
        match c {
            '0'...'7' => {
                if !self.parser().octal {
                    return Err(self.error(
                        Span::new(start, self.span_char().end),
                        ast::ErrorKind::UnsupportedBackreference,
                    ));
                }
                let mut lit = self.parse_octal();
                lit.span.start = start;
                return Ok(Primitive::Literal(lit));
            }
            '8'...'9' if !self.parser().octal => {
                return Err(self.error(
                    Span::new(start, self.span_char().end),
                    ast::ErrorKind::UnsupportedBackreference,
                ));
            }
            'x' | 'u' | 'U' => {
                let mut lit = self.parse_hex()?;
                lit.span.start = start;
                return Ok(Primitive::Literal(lit));
            }
            'p' | 'P' => {
                let mut cls = self.parse_unicode_class()?;
                cls.span.start = start;
                return Ok(Primitive::Unicode(cls));
            }
            'd' | 's' | 'w' | 'D' | 'S' | 'W' => {
                let mut cls = self.parse_perl_class();
                cls.span.start = start;
                return Ok(Primitive::Perl(cls));
            }
            _ => {}
        }

        // Handle all of the one letter sequences inline.
        self.bump();
        let span = Span::new(start, self.pos());
        if is_meta_character(c) {
            return Ok(Primitive::Literal(ast::Literal {
                span: span,
                kind: ast::LiteralKind::Punctuation,
                c: c,
            }));
        }
        let special = |kind, c| Ok(Primitive::Literal(ast::Literal {
            span: span,
            kind: ast::LiteralKind::Special(kind),
            c: c,
        }));
        match c {
            'a' => special(ast::SpecialLiteralKind::Bell, '\x07'),
            'f' => special(ast::SpecialLiteralKind::FormFeed, '\x0C'),
            't' => special(ast::SpecialLiteralKind::Tab, '\t'),
            'n' => special(ast::SpecialLiteralKind::LineFeed, '\n'),
            'r' => special(ast::SpecialLiteralKind::CarriageReturn, '\r'),
            'v' => special(ast::SpecialLiteralKind::VerticalTab, '\x0B'),
            ' ' if self.ignore_whitespace() => {
                special(ast::SpecialLiteralKind::Space, ' ')
            }
            'A' => Ok(Primitive::Assertion(ast::Assertion {
                span: span,
                kind: ast::AssertionKind::StartText,
            })),
            'z' => Ok(Primitive::Assertion(ast::Assertion {
                span: span,
                kind: ast::AssertionKind::EndText,
            })),
            'b' => Ok(Primitive::Assertion(ast::Assertion {
                span: span,
                kind: ast::AssertionKind::WordBoundary,
            })),
            'B' => Ok(Primitive::Assertion(ast::Assertion {
                span: span,
                kind: ast::AssertionKind::NotWordBoundary,
            })),
            _ => Err(self.error(span, ast::ErrorKind::EscapeUnrecognized)),
        }
    }