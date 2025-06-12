fn parse_flags(&self) -> Result<ast::Flags> {
        let mut flags = ast::Flags {
            span: self.span(),
            items: vec![],
        };
        let mut last_was_negation = None;
        while self.char() != ':' && self.char() != ')' {
            if self.char() == '-' {
                last_was_negation = Some(self.span_char());
                let item = ast::FlagsItem {
                    span: self.span_char(),
                    kind: ast::FlagsItemKind::Negation,
                };
                if let Some(i) = flags.add_item(item) {
                    return Err(self.error(
                        self.span_char(),
                        ast::ErrorKind::FlagRepeatedNegation {
                            original: flags.items[i].span,
                        },
                    ));
                }
            } else {
                last_was_negation = None;
                let item = ast::FlagsItem {
                    span: self.span_char(),
                    kind: ast::FlagsItemKind::Flag(self.parse_flag()?),
                };
                if let Some(i) = flags.add_item(item) {
                    return Err(self.error(
                        self.span_char(),
                        ast::ErrorKind::FlagDuplicate {
                            original: flags.items[i].span,
                        },
                    ));
                }
            }
            if !self.bump() {
                return Err(self.error(
                    self.span(),
                    ast::ErrorKind::FlagUnexpectedEof,
                ));
            }
        }
        if let Some(span) = last_was_negation {
            return Err(self.error(span, ast::ErrorKind::FlagDanglingNegation));
        }
        flags.span.end = self.pos();
        Ok(flags)
    }