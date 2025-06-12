fn parse_set_class_range(&self) -> Result<ast::ClassSetItem> {
        let prim1 = self.parse_set_class_item()?;
        self.bump_space();
        if self.is_eof() {
            return Err(self.unclosed_class_error());
        }
        // If the next char isn't a `-`, then we don't have a range.
        // There are two exceptions. If the char after a `-` is a `]`, then
        // `-` is interpreted as a literal `-`. Alternatively, if the char
        // after a `-` is a `-`, then `--` corresponds to a "difference"
        // operation.
        if self.char() != '-'
            || self.peek_space() == Some(']')
            || self.peek_space() == Some('-')
        {
            return prim1.into_class_set_item(self);
        }
        // OK, now we're parsing a range, so bump past the `-` and parse the
        // second half of the range.
        if !self.bump_and_bump_space() {
            return Err(self.unclosed_class_error());
        }
        let prim2 = self.parse_set_class_item()?;
        let range = ast::ClassSetRange {
            span: Span::new(prim1.span().start, prim2.span().end),
            start: prim1.into_class_literal(self)?,
            end: prim2.into_class_literal(self)?,
        };
        if !range.is_valid() {
            return Err(self.error(
                range.span,
                ast::ErrorKind::ClassRangeInvalid,
            ));
        }
        Ok(ast::ClassSetItem::Range(range))
    }