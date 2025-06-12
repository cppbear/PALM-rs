// Answer 0

#[derive(Debug)]
struct Parser {
    input: &'static str,
    position: usize,
}

impl Parser {
    fn char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    fn span_char(&self) -> usize {
        self.position
    }

    fn bump(&mut self) {
        self.position += 1;
    }

    fn parse_perl_class(&mut self) -> ClassPerl {
        let c = self.char();
        let span = self.span_char();
        self.bump();
        let (negated, kind) = match c {
            'd' => (false, ClassPerlKind::Digit),
            'D' => (true, ClassPerlKind::Digit),
            's' => (false, ClassPerlKind::Space),
            'S' => (true, ClassPerlKind::Space),
            'w' => (false, ClassPerlKind::Word),
            'W' => (true, ClassPerlKind::Word),
            c => panic!("expected valid Perl class but got '{}'", c),
        };
        ClassPerl { span: span, kind: kind, negated: negated }
    }
}

#[derive(Debug, PartialEq)]
struct ClassPerl {
    span: usize,
    kind: ClassPerlKind,
    negated: bool,
}

#[derive(Debug, PartialEq)]
enum ClassPerlKind {
    Digit,
    Space,
    Word,
}

#[test]
fn test_parse_perl_class_digit() {
    let mut parser = Parser { input: "\\d", position: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result, ClassPerl { span: 0, kind: ClassPerlKind::Digit, negated: false });
}

#[test]
fn test_parse_perl_class_negated_digit() {
    let mut parser = Parser { input: "\\D", position: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result, ClassPerl { span: 0, kind: ClassPerlKind::Digit, negated: true });
}

#[test]
fn test_parse_perl_class_space() {
    let mut parser = Parser { input: "\\s", position: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result, ClassPerl { span: 0, kind: ClassPerlKind::Space, negated: false });
}

#[test]
fn test_parse_perl_class_negated_space() {
    let mut parser = Parser { input: "\\S", position: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result, ClassPerl { span: 0, kind: ClassPerlKind::Space, negated: true });
}

#[test]
fn test_parse_perl_class_word() {
    let mut parser = Parser { input: "\\w", position: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result, ClassPerl { span: 0, kind: ClassPerlKind::Word, negated: false });
}

#[test]
fn test_parse_perl_class_negated_word() {
    let mut parser = Parser { input: "\\W", position: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result, ClassPerl { span: 0, kind: ClassPerlKind::Word, negated: true });
}

#[test]
#[should_panic(expected = "expected valid Perl class but got 'a'")]
fn test_parse_perl_class_invalid() {
    let mut parser = Parser { input: "\\a", position: 0 };
    parser.parse_perl_class();
}

