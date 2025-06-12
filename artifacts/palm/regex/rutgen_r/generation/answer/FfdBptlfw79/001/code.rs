// Answer 0

#[derive(Debug)]
struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
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
        self.pos // Just returning the position as the span for simplicity
    }

    fn parse_escape(&self) -> Result<Primitive, &'static str> {
        // Mock implementation for escape
        Ok(Primitive::Escape(self.span_char()))
    }
    
    fn parse_primitive(&mut self) -> Result<Primitive, &'static str> {
        match self.char() {
            '\\' => self.parse_escape(),
            '.' => {
                let ast = Primitive::Dot(self.span_char());
                self.bump();
                Ok(ast)
            }
            '^' => {
                let ast = Primitive::Assertion(Assertion {
                    span: self.span_char(),
                    kind: AssertionKind::StartLine,
                });
                self.bump();
                Ok(ast)
            }
            '$' => {
                let ast = Primitive::Assertion(Assertion {
                    span: self.span_char(),
                    kind: AssertionKind::EndLine,
                });
                self.bump();
                Ok(ast)
            }
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

#[derive(Debug)]
enum Primitive {
    Dot(usize),
    Assertion(Assertion),
    Literal(Literal),
    Escape(usize),
}

#[derive(Debug)]
struct Assertion {
    span: usize,
    kind: AssertionKind,
}

#[derive(Debug)]
enum AssertionKind {
    StartLine,
    EndLine,
}

#[derive(Debug)]
struct Literal {
    span: usize,
    kind: LiteralKind,
    c: char,
}

#[derive(Debug)]
enum LiteralKind {
    Verbatim,
}

#[test]
fn test_parse_dot() {
    let mut parser = Parser::new(".");
    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(Primitive::Dot(_)) = result {
    } else {
        panic!("Expected Dot variant");
    }
}

#[test]
fn test_parse_start_line() {
    let mut parser = Parser::new("^");
    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(Primitive::Assertion(assertion)) = result {
        assert_eq!(assertion.kind, AssertionKind::StartLine);
    } else {
        panic!("Expected Assertion variant with StartLine kind");
    }
}

#[test]
fn test_parse_end_line() {
    let mut parser = Parser::new("$");
    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(Primitive::Assertion(assertion)) = result {
        assert_eq!(assertion.kind, AssertionKind::EndLine);
    } else {
        panic!("Expected Assertion variant with EndLine kind");
    }
}

#[test]
fn test_parse_escape() {
    let mut parser = Parser::new("\\");
    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(Primitive::Escape(_)) = result {
    } else {
        panic!("Expected Escape variant");
    }
}

#[test]
fn test_parse_literal() {
    let mut parser = Parser::new("a");
    let result = parser.parse_primitive();
    assert!(result.is_ok());
    if let Ok(Primitive::Literal(literal)) = result {
        assert_eq!(literal.c, 'a');
        assert_eq!(literal.kind, LiteralKind::Verbatim);
    } else {
        panic!("Expected Literal variant");
    }
}

