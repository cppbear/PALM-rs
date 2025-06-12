// Answer 0

#[test]
fn test_parse_primitive_escape() {
    struct DummyParser {
        input: &'static str,
        current_pos: usize,
    }

    impl DummyParser {
        fn char(&self) -> char {
            self.input.chars().nth(self.current_pos).unwrap()
        }

        fn bump(&mut self) {
            self.current_pos += 1;
        }

        fn span_char(&self) -> Span {
            Span{ start: self.current_pos as Position, end: self.current_pos as Position + 1 }
        }

        fn parse_escape(&self) -> Result<Primitive> {
            // Simulate an escape character parsing returning a literal
            let span = self.span_char();
            Ok(Primitive::Literal(Literal {
                span,
                kind: LiteralKind::Special(ast::SpecialLiteralKind::Tab),
                c: '\t',
            }))
        }

        fn parse_primitive(&mut self) -> Result<Primitive> {
            match self.char() {
                '\\' => self.parse_escape(),
                _ => {
                    let ast = Primitive::Literal(Literal {
                        span: self.span_char(),
                        kind: LiteralKind::Verbatim,
                        c: self.char(),
                    });
                    self.bump();
                    Ok(ast)
                }
            }
        }
    }

    let mut parser = DummyParser { input: "\\t", current_pos: 0 };
    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(primitive) = result {
        if let Primitive::Literal(lit) = primitive {
            assert_eq!(lit.kind, LiteralKind::Special(ast::SpecialLiteralKind::Tab));
            assert_eq!(lit.c, '\t');
            assert_eq!(lit.span.start, 0);
            assert_eq!(lit.span.end, 1);
        }
    }

    parser.current_pos = 0; // Resetting position for another test
    parser.input = "\\n"; // Testing with another escape character
    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(primitive) = result {
        if let Primitive::Literal(lit) = primitive {
            assert_eq!(lit.kind, LiteralKind::Special(ast::SpecialLiteralKind::LineFeed));
            assert_eq!(lit.c, '\n');
            assert_eq!(lit.span.start, 0);
            assert_eq!(lit.span.end, 1);
        }
    }
}

#[test]
fn test_parse_primitive_literal() {
    struct DummyParser {
        input: &'static str,
        current_pos: usize,
    }

    impl DummyParser {
        fn char(&self) -> char {
            self.input.chars().nth(self.current_pos).unwrap()
        }

        fn bump(&mut self) {
            self.current_pos += 1;
        }

        fn span_char(&self) -> Span {
            Span{ start: self.current_pos as Position, end: self.current_pos as Position + 1 }
        }

        fn parse_primitive(&mut self) -> Result<Primitive> {
            match self.char() {
                '\\' => Err(ast::Error::new()), // Simulated not actually handling escapes here
                c => {
                    let ast = Primitive::Literal(Literal {
                        span: self.span_char(),
                        kind: LiteralKind::Verbatim,
                        c,
                    });
                    self.bump();
                    Ok(ast)
                }
            }
        }
    }

    let mut parser = DummyParser { input: "a", current_pos: 0 };
    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(primitive) = result {
        if let Primitive::Literal(lit) = primitive {
            assert_eq!(lit.kind, LiteralKind::Verbatim);
            assert_eq!(lit.c, 'a');
            assert_eq!(lit.span.start, 0);
            assert_eq!(lit.span.end, 1);
        }
    }

    parser.current_pos = 0; // Resetting position for another test
    parser.input = "b"; // Testing with another literal character
    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(primitive) = result {
        if let Primitive::Literal(lit) = primitive {
            assert_eq!(lit.kind, LiteralKind::Verbatim);
            assert_eq!(lit.c, 'b');
            assert_eq!(lit.span.start, 0);
            assert_eq!(lit.span.end, 1);
        }
    }
}

