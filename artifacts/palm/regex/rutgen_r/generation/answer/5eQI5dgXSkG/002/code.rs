// Answer 0

#[derive(Clone, Copy)]
struct MockParser {
    current_char: char,
    current_span: usize,
}

impl MockParser {
    fn char(&self) -> char {
        self.current_char
    }
    
    fn span_char(&self) -> usize {
        self.current_span
    }

    fn bump(&mut self) {
        // Simulate moving to the next character in an actual implementation
    }
}

mod ast {
    #[derive(Debug, PartialEq)]
    pub enum ClassPerlKind {
        Digit,
        Space,
        Word,
    }

    #[derive(Debug, PartialEq)]
    pub struct ClassPerl {
        pub span: usize,
        pub kind: ClassPerlKind,
        pub negated: bool,
    }
}

#[test]
fn test_parse_perl_class_word() {
    let mut parser = MockParser {
        current_char: 'w',
        current_span: 0,
    };
    
    let result = parser.parse_perl_class();
    
    assert_eq!(result, ast::ClassPerl { span: 0, kind: ast::ClassPerlKind::Word, negated: false });
}

#[test]
fn test_parse_perl_class_negated_word() {
    let mut parser = MockParser {
        current_char: 'W',
        current_span: 1,
    };

    let result = parser.parse_perl_class();

    assert_eq!(result, ast::ClassPerl { span: 1, kind: ast::ClassPerlKind::Word, negated: true });
}

#[test]
#[should_panic(expected = "expected valid Perl class")]
fn test_parse_perl_class_invalid() {
    let mut parser = MockParser {
        current_char: 'x',
        current_span: 2,
    };

    parser.parse_perl_class();
}

