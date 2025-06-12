fn parse_unicode_class(&self) -> Result<ast::ClassUnicode> {
        assert!(self.char() == 'p' || self.char() == 'P');

        let mut scratch = self.parser().scratch.borrow_mut();
        scratch.clear();

        let negated = self.char() == 'P';
        if !self.bump_and_bump_space() {
            return Err(self.error(
                self.span(),
                ast::ErrorKind::EscapeUnexpectedEof,
            ));
        }
        let (start, kind) =
            if self.char() == '{' {
                let start = self.span_char().end;
                while self.bump_and_bump_space() && self.char() != '}' {
                    scratch.push(self.char());
                }
                if self.is_eof() {
                    return Err(self.error(
                        self.span(),
                        ast::ErrorKind::EscapeUnexpectedEof,
                    ));
                }
                assert_eq!(self.char(), '}');
                self.bump();

                let name = scratch.as_str();
                if let Some(i) = name.find("!=") {
                    (start, ast::ClassUnicodeKind::NamedValue {
                        op: ast::ClassUnicodeOpKind::NotEqual,
                        name: name[..i].to_string(),
                        value: name[i+2..].to_string(),
                    })
                } else if let Some(i) = name.find(':') {
                    (start, ast::ClassUnicodeKind::NamedValue {
                        op: ast::ClassUnicodeOpKind::Colon,
                        name: name[..i].to_string(),
                        value: name[i+1..].to_string(),
                    })
                } else if let Some(i) = name.find('=') {
                    (start, ast::ClassUnicodeKind::NamedValue {
                        op: ast::ClassUnicodeOpKind::Equal,
                        name: name[..i].to_string(),
                        value: name[i+1..].to_string(),
                    })
                } else {
                    (start, ast::ClassUnicodeKind::Named(name.to_string()))
                }
            } else {
                let start = self.pos();
                let c = self.char();
                self.bump_and_bump_space();
                let kind = ast::ClassUnicodeKind::OneLetter(c);
                (start, kind)
            };
        Ok(ast::ClassUnicode {
            span: Span::new(start, self.pos()),
            negated: negated,
            kind: kind,
        })
    }