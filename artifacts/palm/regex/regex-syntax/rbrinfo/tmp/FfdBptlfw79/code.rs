fn parse_primitive(&self) -> Result<Primitive> {
        match self.char() {
            '\\' => self.parse_escape(),
            '.' => {
                let ast = Primitive::Dot(self.span_char());
                self.bump();
                Ok(ast)
            }
            '^' => {
                let ast = Primitive::Assertion(ast::Assertion {
                    span: self.span_char(),
                    kind: ast::AssertionKind::StartLine,
                });
                self.bump();
                Ok(ast)
            }
            '$' => {
                let ast = Primitive::Assertion(ast::Assertion {
                    span: self.span_char(),
                    kind: ast::AssertionKind::EndLine,
                });
                self.bump();
                Ok(ast)
            }
            c => {
                let ast = Primitive::Literal(ast::Literal {
                    span: self.span_char(),
                    kind: ast::LiteralKind::Verbatim,
                    c: c,
                });
                self.bump();
                Ok(ast)
            }
        }
    }