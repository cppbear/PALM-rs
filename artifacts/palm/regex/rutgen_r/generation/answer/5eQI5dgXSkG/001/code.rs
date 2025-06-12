// Answer 0

#[derive(Debug)]
struct Parser {
    input: Vec<char>,
    position: usize,
}

impl Parser {
    fn char(&self) -> char {
        self.input[self.position]
    }

    fn span_char(&self) -> usize {
        self.position
    }

    fn bump(&mut self) {
        self.position += 1;
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
    #[derive(Debug)]
    pub struct ClassPerl {
        pub span: usize,
        pub kind: ClassPerlKind,
        pub negated: bool,
    }

    #[derive(Debug)]
    pub enum ClassPerlKind {
        Digit,
        Space,
        Word,
    }
}

#[test]
fn test_parse_perl_class_digit() {
    let mut parser = Parser { input: vec!['d'], position: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result.negated, false);
    assert_eq!(result.kind, ast::ClassPerlKind::Digit);
    assert_eq!(result.span, 0);
}

#[test]
fn test_parse_perl_class_negated_digit() {
    let mut parser = Parser { input: vec!['D'], position: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result.negated, true);
    assert_eq!(result.kind, ast::ClassPerlKind::Digit);
    assert_eq!(result.span, 0);
}

#[test]
fn test_parse_perl_class_space() {
    let mut parser = Parser { input: vec!['s'], position: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result.negated, false);
    assert_eq!(result.kind, ast::ClassPerlKind::Space);
    assert_eq!(result.span, 0);
}

#[test]
fn test_parse_perl_class_negated_space() {
    let mut parser = Parser { input: vec!['S'], position: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result.negated, true);
    assert_eq!(result.kind, ast::ClassPerlKind::Space);
    assert_eq!(result.span, 0);
}

#[test]
fn test_parse_perl_class_word() {
    let mut parser = Parser { input: vec!['w'], position: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result.negated, false);
    assert_eq!(result.kind, ast::ClassPerlKind::Word);
    assert_eq!(result.span, 0);
}

#[test]
fn test_parse_perl_class_negated_word() {
    let mut parser = Parser { input: vec!['W'], position: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result.negated, true);
    assert_eq!(result.kind, ast::ClassPerlKind::Word);
    assert_eq!(result.span, 0);
}

#[should_panic(expected = "expected valid Perl class but got 'x'")] // Example of a panic case
#[test]
fn test_parse_perl_class_invalid() {
    let mut parser = Parser { input: vec!['x'], position: 0 };
    parser.parse_perl_class();
}

