fn parse_set_class_open(
        &self,
    ) -> Result<(ast::ClassBracketed, ast::ClassSetUnion)> {
        assert_eq!(self.char(), '[');
        let start = self.pos();
        if !self.bump_and_bump_space() {
            return Err(self.error(
                Span::new(start, self.pos()),
                ast::ErrorKind::ClassUnclosed,
            ));
        }

        let negated =
            if self.char() != '^' {
                false
            } else {
                if !self.bump_and_bump_space() {
                    return Err(self.error(
                        Span::new(start, self.pos()),
                        ast::ErrorKind::ClassUnclosed,
                    ));
                }
                true
            };
        // Accept any number of `-` as literal `-`.
        let mut union = ast::ClassSetUnion {
            span: self.span(),
            items: vec![],
        };
        while self.char() == '-' {
            union.push(ast::ClassSetItem::Literal(ast::Literal {
                span: self.span_char(),
                kind: ast::LiteralKind::Verbatim,
                c: '-',
            }));
            if !self.bump_and_bump_space() {
                return Err(self.error(
                    Span::new(start, self.pos()),
                    ast::ErrorKind::ClassUnclosed,
                ));
            }
        }
        // If `]` is the *first* char in a set, then interpret it as a literal
        // `]`. That is, an empty class is impossible to write.
        if union.items.is_empty() && self.char() == ']' {
            union.push(ast::ClassSetItem::Literal(ast::Literal {
                span: self.span_char(),
                kind: ast::LiteralKind::Verbatim,
                c: ']',
            }));
            if !self.bump_and_bump_space() {
                return Err(self.error(
                    Span::new(start, self.pos()),
                    ast::ErrorKind::ClassUnclosed,
                ));
            }
        }
        let set = ast::ClassBracketed {
            span: Span::new(start, self.pos()),
            negated: negated,
            kind: ast::ClassSet::union(ast::ClassSetUnion {
                span: Span::new(union.span.start, union.span.start),
                items: vec![],
            }),
        };
        Ok((set, union))
    }