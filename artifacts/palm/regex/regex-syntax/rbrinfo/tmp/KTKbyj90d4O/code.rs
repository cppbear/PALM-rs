fn parse_capture_name(
        &self,
        capture_index: u32,
    ) -> Result<ast::CaptureName> {
        if self.is_eof() {
            return Err(self.error(
                self.span(),
                ast::ErrorKind::GroupNameUnexpectedEof,
            ));
        }
        let start = self.pos();
        loop {
            if self.char() == '>' {
                break;
            }
            if !is_capture_char(self.char(), self.pos() == start) {
                return Err(self.error(
                    self.span_char(),
                    ast::ErrorKind::GroupNameInvalid,
                ));
            }
            if !self.bump() {
                break;
            }
        }
        let end = self.pos();
        if self.is_eof() {
            return Err(self.error(
                self.span(),
                ast::ErrorKind::GroupNameUnexpectedEof,
            ));
        }
        assert_eq!(self.char(), '>');
        self.bump();
        let name = &self.pattern()[start.offset..end.offset];
        if name.is_empty() {
            return Err(self.error(
                Span::new(start, start),
                ast::ErrorKind::GroupNameEmpty,
            ));
        }
        let capname = ast::CaptureName {
            span: Span::new(start, end),
            name: name.to_string(),
            index: capture_index,
        };
        self.add_capture_name(&capname)?;
        Ok(capname)
    }