// Answer 0

#[derive(Debug)]
struct DummyParser {
    input: Vec<char>,
    index: usize,
}

impl DummyParser {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            index: 0,
        }
    }

    fn char(&self) -> char {
        self.input[self.index]
    }

    fn span_char(&self) -> usize {
        self.index
    }

    fn bump(&mut self) {
        self.index += 1;
    }

    fn parse_perl_class(&mut self) -> ast::ClassPerl {
        let c = self.char();
        let span = self.span_char();
        self.bump();
        let (negated, kind) = match c {
            'd' => (false, ast::ClassPerlKind::Digit),
            'D' => (true, ast::ClassPerlKind::Digit),
            's' => (false, ast::ClassPerlKind::Space),
            'S' => (true, ast::ClassPerlKind::Space),
            'w' => (false, ast::ClassPerlKind::Word),
            'W' => (true, ast::ClassPerlKind::Word),
            c => panic!("expected valid Perl class but got '{}'", c),
        };
        ast::ClassPerl { span: span, kind: kind, negated: negated }
    }
}

mod ast {
    #[derive(Debug, PartialEq)]
    pub struct ClassPerl {
        pub span: usize,
        pub kind: ClassPerlKind,
        pub negated: bool,
    }

    #[derive(Debug, PartialEq)]
    pub enum ClassPerlKind {
        Digit,
        Space,
        Word,
    }
}

#[test]
fn test_parse_perl_class_digit() {
    let mut parser = DummyParser::new(r"\d");
    let result = parser.parse_perl_class();
    assert_eq!(result, ast::ClassPerl { span: 0, kind: ast::ClassPerlKind::Digit, negated: false });
}

#[test]
fn test_parse_perl_class_space() {
    let mut parser = DummyParser::new(r"\s");
    let result = parser.parse_perl_class();
    assert_eq!(result, ast::ClassPerl { span: 0, kind: ast::ClassPerlKind::Space, negated: false });
}

#[test]
fn test_parse_perl_class_word() {
    let mut parser = DummyParser::new(r"\w");
    let result = parser.parse_perl_class();
    assert_eq!(result, ast::ClassPerl { span: 0, kind: ast::ClassPerlKind::Word, negated: false });
}

#[test]
fn test_parse_perl_class_negated_word() {
    let mut parser = DummyParser::new(r"\W");
    let result = parser.parse_perl_class();
    assert_eq!(result, ast::ClassPerl { span: 0, kind: ast::ClassPerlKind::Word, negated: true });
}

#[test]
fn test_parse_perl_class_invalid() {
    let mut parser = DummyParser::new(r"\x");
    let result = std::panic::catch_unwind(|| parser.parse_perl_class());
    assert!(result.is_err());
}

