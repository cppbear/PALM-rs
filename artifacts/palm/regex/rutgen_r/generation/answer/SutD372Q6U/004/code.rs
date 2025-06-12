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
        *self.input.get(self.position).unwrap_or(&'\0')
    }

    fn bump(&mut self) {
        self.position += 1;
    }

    fn bump_space(&mut self) {
        while self.char().is_whitespace() {
            self.bump();
        }
    }

    fn is_lookaround_prefix(&self) -> bool {
        false
    }

    fn bump_if(&mut self, prefix: &str) -> bool {
        if self.input[self.position..].starts_with(prefix.chars().collect::<Vec<_>>().as_slice()) {
            self.position += prefix.len();
            true
        } else {
            false
        }
    }

    fn next_capture_index(&self, _: usize) -> Result<usize, ()> {
        Ok(0)
    }

    fn parse_capture_name(&self, _: usize) -> Result<String, ()> {
        Ok("name".to_string())
    }

    fn span_char(&self) -> usize {
        self.position
    }

    fn span(&self) -> usize {
        self.position
    }

    fn pos(&self) -> usize {
        self.position
    }

    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }
}

#[test]
fn test_parse_group_with_capture_name() {
    let mut parser = MockParser::new("(name)");
    let result = parser.parse_group();
    
    assert!(result.is_ok());
    
    if let Ok(Either::Right(group)) = result {
        assert_eq!(group.kind, ast::GroupKind::CaptureName("name".to_string()));
    }
}

#[test]
#[should_panic]
fn test_parse_group_with_unsupported_lookaround() {
    let mut parser = MockParser::new("(?=foo)");
    parser.bump_if("(?=");
    parser.bump();
    parser.bump_space();
    assert!(parser.is_lookaround_prefix());
    parser.parse_group();
}

#[test]
#[should_panic]
fn test_parse_group_with_empty_flags() {
    let mut parser = MockParser::new("(?)");
    parser.bump_if("(?") // Mock grouping
        .then(|| { parser.bump(); }); // Don't bump space here for test
    let result = parser.parse_group();
    
    assert!(result.is_err());
}

