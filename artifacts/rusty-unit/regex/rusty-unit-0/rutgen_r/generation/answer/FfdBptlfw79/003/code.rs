// Answer 0

#[test]
fn test_parse_primitive_start_line_assertion() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }
    
    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }
        
        fn bump(&mut self) {
            self.pos += 1;
        }
        
        fn span_char(&self) -> usize {
            self.pos
        }
        
        fn parse_escape(&self) -> Result<Primitive> {
            // Not testing escape sequences in this test
            Err("Not implemented".into())
        }
    }
    
    #[derive(Debug)]
    enum Primitive {
        Assertion(ast::Assertion),
    }

    mod ast {
        #[derive(Debug)]
        pub struct Assertion {
            pub span: usize,
            pub kind: AssertionKind,
        }
        
        #[derive(Debug)]
        pub enum AssertionKind {
            StartLine,
            EndLine,
        }
    }
    
    let mut parser = MockParser::new("^a");
    match parser.parse_primitive() {
        Ok(Primitive::Assertion(assertion)) => {
            assert_eq!(assertion.kind, ast::AssertionKind::StartLine);
            assert_eq!(assertion.span, 0);
        },
        _ => panic!("Expected an assertion for start line."),
    }
}

#[test]
fn test_parse_primitive_end_line_assertion() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }
    
    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.pos]
        }
        
        fn bump(&mut self) {
            self.pos += 1;
        }
        
        fn span_char(&self) -> usize {
            self.pos
        }
        
        fn parse_escape(&self) -> Result<Primitive> {
            // Not testing escape sequences in this test
            Err("Not implemented".into())
        }
    }
    
    #[derive(Debug)]
    enum Primitive {
        Assertion(ast::Assertion),
    }

    mod ast {
        #[derive(Debug)]
        pub struct Assertion {
            pub span: usize,
            pub kind: AssertionKind,
        }
        
        #[derive(Debug)]
        pub enum AssertionKind {
            StartLine,
            EndLine,
        }
    }
    
    let mut parser = MockParser::new("$a");
    match parser.parse_primitive() {
        Ok(Primitive::Assertion(assertion)) => {
            assert_eq!(assertion.kind, ast::AssertionKind::EndLine);
            assert_eq!(assertion.span, 0);
        },
        _ => panic!("Expected an assertion for end line."),
    }
}

