// Answer 0

#[test]
fn test_parse_set_class_item_escape() {
    struct Parser {
        input: Vec<char>,
        position: usize,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.position).unwrap_or(&'\0')
        }

        fn bump(&mut self) {
            self.position += 1;
        }

        fn parse_escape(&self) -> Result<Primitive, String> {
            if self.char() == 'n' {
                Ok(Primitive::Literal(ast::Literal {
                    span: 0, // Simplified for testing
                    kind: ast::LiteralKind::Verbatim,
                    c: '\n',
                }))
            } else {
                Err("Invalid escape sequence".to_string())
            }
        }

        fn parse_set_class_item(&mut self) -> Result<Primitive, String> {
            if self.char() == '\\' {
                self.bump();
                self.parse_escape()
            } else {
                let x = Primitive::Literal(ast::Literal {
                    span: 0,
                    kind: ast::LiteralKind::Verbatim,
                    c: self.char(),
                });
                self.bump();
                Ok(x)
            }
        }
    }

    mod ast {
        pub struct Literal {
            pub span: usize,
            pub kind: LiteralKind,
            pub c: char,
        }

        #[derive(Debug)]
        pub enum LiteralKind {
            Verbatim,
        }
    }

    #[derive(Debug)]
    pub enum Primitive {
        Literal(ast::Literal),
    }

    let mut parser = Parser::new("\\n");
    let result = parser.parse_set_class_item();
    assert!(result.is_ok());
    if let Ok(Primitive::Literal(lit)) = result {
        assert_eq!(lit.c, '\n');
        assert_eq!(lit.kind, ast::LiteralKind::Verbatim);
    } else {
        panic!("Expected Ok result, got an Err");
    }
}

