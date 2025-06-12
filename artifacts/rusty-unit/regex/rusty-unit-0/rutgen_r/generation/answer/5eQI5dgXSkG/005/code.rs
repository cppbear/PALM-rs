// Answer 0

#[derive(Debug)]
struct MockParser {
    input: Vec<char>,
    position: usize,
}

impl MockParser {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn char(&self) -> char {
        self.input[self.position]
    }

    fn span_char(&self) -> usize {
        self.position
    }

    fn bump(&mut self) {
        self.position += 1;
    }
}

mod ast {
    #[derive(Debug)]
    pub enum ClassPerlKind {
        Digit,
        Space,
        Word,
    }

    #[derive(Debug)]
    pub struct ClassPerl {
        pub span: usize,
        pub kind: ClassPerlKind,
        pub negated: bool,
    }
}

#[test]
fn test_parse_perl_class_space() {
    let mut parser = MockParser::new(r"\s");
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Space);
    assert_eq!(result.negated, false);
}

#[test]
fn test_parse_perl_class_negated_space() {
    let mut parser = MockParser::new(r"\S");
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Space);
    assert_eq!(result.negated, true);
}

#[test]
fn test_parse_perl_class_panic_on_invalid_char() {
    let mut parser = MockParser::new(r"\x");
    let panic_result = std::panic::catch_unwind(|| {
        parser.parse_perl_class();
    });
    assert!(panic_result.is_err());
}

