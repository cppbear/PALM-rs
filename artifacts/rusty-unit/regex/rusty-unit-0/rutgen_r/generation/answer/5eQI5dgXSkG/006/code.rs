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
        *self.input.get(self.position).expect("No more chars")
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
fn test_parse_digit_negated() {
    let mut parser = MockParser::new(r"\D");
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Digit);
    assert!(result.negated);
    assert_eq!(result.span, 0);
}

#[test]
fn test_parse_digit() {
    let mut parser = MockParser::new(r"\d");
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Digit);
    assert!(!result.negated);
    assert_eq!(result.span, 0);
}

#[test]
fn test_parse_space_negated() {
    let mut parser = MockParser::new(r"\S");
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Space);
    assert!(result.negated);
    assert_eq!(result.span, 0);
}

#[test]
fn test_parse_space() {
    let mut parser = MockParser::new(r"\s");
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Space);
    assert!(!result.negated);
    assert_eq!(result.span, 0);
}

#[test]
fn test_parse_word_negated() {
    let mut parser = MockParser::new(r"\W");
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Word);
    assert!(result.negated);
    assert_eq!(result.span, 0);
}

#[test]
fn test_parse_word() {
    let mut parser = MockParser::new(r"\w");
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Word);
    assert!(!result.negated);
    assert_eq!(result.span, 0);
}

#[should_panic(expected = "expected valid Perl class but got 'x'")]
#[test]
fn test_parse_invalid_character() {
    let mut parser = MockParser::new(r"\x");
    parser.parse_perl_class();
}

