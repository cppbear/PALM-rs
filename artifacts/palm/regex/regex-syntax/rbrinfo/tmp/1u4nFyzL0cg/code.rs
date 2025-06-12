fn parse_set_class(&self) -> Result<ast::Class> {
        assert_eq!(self.char(), '[');

        let mut union = ast::ClassSetUnion {
            span: self.span(),
            items: vec![],
        };
        loop {
            self.bump_space();
            if self.is_eof() {
                return Err(self.unclosed_class_error());
            }
            match self.char() {
                '[' => {
                    // If we've already parsed the opening bracket, then
                    // attempt to treat this as the beginning of an ASCII
                    // class. If ASCII class parsing fails, then the parser
                    // backs up to `[`.
                    if !self.parser().stack_class.borrow().is_empty() {
                        if let Some(cls) = self.maybe_parse_ascii_class() {
                            union.push(ast::ClassSetItem::Ascii(cls));
                            continue;
                        }
                    }
                    union = self.push_class_open(union)?;
                }
                ']' => {
                    match self.pop_class(union)? {
                        Either::Left(nested_union) => { union = nested_union; }
                        Either::Right(class) => return Ok(class),
                    }
                }
                '&' if self.peek() == Some('&') => {
                    assert!(self.bump_if("&&"));
                    union = self.push_class_op(
                        ast::ClassSetBinaryOpKind::Intersection, union);
                }
                '-' if self.peek() == Some('-') => {
                    assert!(self.bump_if("--"));
                    union = self.push_class_op(
                        ast::ClassSetBinaryOpKind::Difference, union);
                }
                '~' if self.peek() == Some('~') => {
                    assert!(self.bump_if("~~"));
                    union = self.push_class_op(
                        ast::ClassSetBinaryOpKind::SymmetricDifference, union);
                }
                _ => {
                    union.push(self.parse_set_class_range()?);
                }
            }
        }
    }