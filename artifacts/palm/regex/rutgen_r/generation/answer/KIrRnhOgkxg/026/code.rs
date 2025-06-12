// Answer 0

#[test]
fn test_maybe_parse_ascii_class_invalid_due_to_extra_colon() {
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

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input.get(self.pos).cloned().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn pattern(&self) -> Vec<char> {
            self.input.clone()
        }

        fn offset(&self) -> usize {
            self.pos
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if chars.contains(self.char()) {
                self.bump();
                true
            } else {
                false
            }
        }
    }

    struct AstClassAscii {
        span: (usize, usize),
        kind: usize,
        negated: bool,
    }

    impl AstClassAscii {
        fn from_name(name: &str) -> Option<usize> {
            match name {
                "alnum" => Some(0),
                "lower" => Some(1),
                _ => None,
            }
        }
    }

    let mut parser = Parser::new("[[:loower:]]");

    assert_eq!(parser.char(), '[');
    let start = parser.offset();
    let mut negated = false;
    assert!(parser.bump());
    assert_eq!(parser.char(), ':');
    assert!(parser.bump());
    if parser.char() == '^' {
        negated = true;
        assert!(parser.bump());
    }
    let name_start = parser.offset();
    while parser.char() != ':' && parser.bump() {}

    assert!(!parser.is_eof());
    let name = &parser.pattern()[name_start..parser.offset()];
    assert!(!parser.bump_if(":]"));

    let kind = AstClassAscii::from_name(name).map(|k| k).or_else(|| {
        parser.pos = start; // Simulate contents of `self.parser().pos.set(start)`
        None
    });

    assert_eq!(kind, None);
}

#[test]
fn test_maybe_parse_ascii_class_invalid_due_to_eof() {
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

        fn bump(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn char(&self) -> char {
            self.input.get(self.pos).cloned().unwrap_or('\0')
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn pattern(&self) -> Vec<char> {
            self.input.clone()
        }

        fn offset(&self) -> usize {
            self.pos
        }

        fn bump_if(&mut self, chars: &str) -> bool {
            if chars.contains(self.char()) {
                self.bump();
                true
            } else {
                false
            }
        }
    }

    struct AstClassAscii {
        span: (usize, usize),
        kind: usize,
        negated: bool,
    }

    impl AstClassAscii {
        fn from_name(name: &str) -> Option<usize> {
            match name {
                "alnum" => Some(0),
                "lower" => Some(1),
                _ => None,
            }
        }
    }

    let mut parser = Parser::new("[[:alnum:");

    assert_eq!(parser.char(), '[');
    let start = parser.offset();
    let mut negated = false;
    assert!(parser.bump());
    assert_eq!(parser.char(), ':');
    assert!(parser.bump());
    if parser.char() == '^' {
        negated = true;
        assert!(parser.bump());
    }
    let name_start = parser.offset();
    while parser.char() != ':' && parser.bump() {}

    assert!(parser.is_eof());
    let name = &parser.pattern()[name_start..parser.offset()];
    parser.pos = start; // Simulate contents of `self.parser().pos.set(start)`

    let kind = AstClassAscii::from_name(name).map(|k| k).or_else(|| None);

    assert_eq!(kind, None);
}

