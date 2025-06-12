// Answer 0

#[derive(Debug)]
struct MockParser {
    input: Vec<char>,
    pos: usize,
}

impl MockParser {
    fn new(input: &str) -> Self {
        MockParser {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn char(&self) -> char {
        self.input[self.pos]
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn bump(&mut self) {
        self.pos += 1;
    }

    fn bump_and_bump_space(&mut self) -> bool {
        if self.pos < self.input.len() {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn parse_decimal(&self) -> Result<usize, ()> {
        Ok(1) // Simulating a successful parse of a decimal
    }

    fn error(&self, _span: Span, _kind: ast::ErrorKind) -> () {
        panic!("Error occurred");
    }
}

#[derive(Debug)]
struct Span;

impl Span {
    fn new(start: usize, end: usize) -> Self {
        Span
    }
}

#[derive(Debug)]
struct Ast;

#[derive(Debug)]
struct ast {
    pub asts: Vec<Ast>,
}

impl ast {
    fn new() -> Self {
        ast { asts: Vec::new() }
    }
}

#[derive(Debug)]
struct Result<T>(T);

#[test]
fn test_parse_counted_repetition_valid() {
    let mut concat = ast::new();
    concat.asts.push(Ast);

    let mut parser = MockParser::new("{1,}"); // Valid input for testing

    let result = parser.parse_counted_repetition(concat);
    match result {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Should not return an error"),
    }
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_invalid_char() {
    let mut concat = ast::new();
    concat.asts.push(Ast);

    let mut parser = MockParser::new("1,}"); // Invalid input, does not start with '{'

    parser.parse_counted_repetition(concat);
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_unclosed() {
    let mut concat = ast::new();
    concat.asts.push(Ast);

    let mut parser = MockParser::new("{1,"); // Unclosed input

    parser.parse_counted_repetition(concat);
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_invalid_range() {
    let mut concat = ast::new();
    concat.asts.push(Ast);

    let mut parser = MockParser::new("{1,2a}"); // Invalid range with non-numeric

    parser.parse_counted_repetition(concat);
}

