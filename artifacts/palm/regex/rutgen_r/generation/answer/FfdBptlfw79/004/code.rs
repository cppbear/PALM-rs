// Answer 0

#[test]
fn test_parse_dot() {
    struct TestParser {
        input: Vec<char>,
        position: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                position: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.position]
        }

        fn bump(&mut self) {
            self.position += 1;
        }

        fn span_char(&self) -> usize {
            self.position
        }

        fn parse_escape(&self) -> Result<Primitive> {
            unimplemented!()
        }

        fn parse_primitive(&mut self) -> Result<Primitive> {
            match self.char() {
                '.' => {
                    let ast = Primitive::Dot(self.span_char());
                    self.bump();
                    Ok(ast)
                }
                // Include other branches for completeness
                _ => unimplemented!()
            }
        }
    }

    #[derive(Debug)]
    enum Primitive {
        Dot(usize),
        // Other variants would be added here
    }

    let mut parser = TestParser::new(".");
    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(ast) = result {
        match ast {
            Primitive::Dot(span) => assert_eq!(span, 0),
            _ => panic!("Expected a Dot primitive"),
        }
    }
}

