// Answer 0

#[derive(Debug)]
struct CharClassParser {
    current_char: char,
    span_value: usize,
}

impl CharClassParser {
    fn char(&self) -> char {
        self.current_char
    }

    fn span_char(&self) -> usize {
        self.span_value
    }

    fn bump(&mut self) {
        // Simulate moving to the next character
        self.current_char = '\0'; // Placeholder for simplicity
        self.span_value += 1; // Advance the span
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
            _ => panic!("expected valid Perl class but got '{}'", c),
        };
        ClassPerl { span, kind, negated }
    }
}

#[derive(Debug)]
struct ClassPerl {
    span: usize,
    kind: ClassPerlKind,
    negated: bool,
}

#[derive(Debug)]
enum ClassPerlKind {
    Digit,
    Space,
    Word,
}

#[test]
fn test_parse_perl_class_digit() {
    let mut parser = CharClassParser { current_char: 'd', span_value: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result.span, 1);
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert_eq!(result.negated, false);
}

#[test]
fn test_parse_perl_class_negated_digit() {
    let mut parser = CharClassParser { current_char: 'D', span_value: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result.span, 1);
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert_eq!(result.negated, true);
}

#[test]
fn test_parse_perl_class_space() {
    let mut parser = CharClassParser { current_char: 's', span_value: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result.span, 1);
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert_eq!(result.negated, false);
}

#[test]
fn test_parse_perl_class_negated_space() {
    let mut parser = CharClassParser { current_char: 'S', span_value: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result.span, 1);
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert_eq!(result.negated, true);
}

#[test]
fn test_parse_perl_class_word() {
    let mut parser = CharClassParser { current_char: 'w', span_value: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result.span, 1);
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert_eq!(result.negated, false);
}

#[test]
fn test_parse_perl_class_negated_word() {
    let mut parser = CharClassParser { current_char: 'W', span_value: 0 };
    let result = parser.parse_perl_class();
    assert_eq!(result.span, 1);
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert_eq!(result.negated, true);
}

#[should_panic(expected = "expected valid Perl class but got 'x'")]
#[test]
fn test_parse_perl_class_invalid() {
    let mut parser = CharClassParser { current_char: 'x', span_value: 0 };
    parser.parse_perl_class();
}

