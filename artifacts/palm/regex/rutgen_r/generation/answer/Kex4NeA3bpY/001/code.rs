// Answer 0

#[test]
fn test_is_eof_at_end_of_pattern() {
    struct DummyParser {
        position: usize,
        pattern: String,
    }

    impl DummyParser {
        fn offset(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn new(pattern: String) -> Self {
            Self {
                position: pattern.len(),
                pattern,
            }
        }
    }

    let parser = DummyParser::new("abc".to_string());
    assert!(parser.is_eof());
}

#[test]
fn test_is_eof_not_at_end_of_pattern() {
    struct DummyParser {
        position: usize,
        pattern: String,
    }

    impl DummyParser {
        fn offset(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn new(position: usize, pattern: String) -> Self {
            Self {
                position,
                pattern,
            }
        }
    }

    // Test with position not equal to the length of the pattern
    let parser = DummyParser::new(2, "abc".to_string());
    assert!(!parser.is_eof());
}

#[test]
fn test_is_eof_with_empty_pattern() {
    struct DummyParser {
        position: usize,
        pattern: String,
    }

    impl DummyParser {
        fn offset(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn new(pattern: String) -> Self {
            Self {
                position: pattern.len(),
                pattern,
            }
        }
    }

    let parser = DummyParser::new("".to_string());
    assert!(parser.is_eof());
}

#[test]
fn test_is_eof_with_position_one_less_than_pattern_length() {
    struct DummyParser {
        position: usize,
        pattern: String,
    }

    impl DummyParser {
        fn offset(&self) -> usize {
            self.position
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn new(position: usize, pattern: String) -> Self {
            Self {
                position,
                pattern,
            }
        }
    }

    let parser = DummyParser::new(2, "xyz".to_string());
    assert!(!parser.is_eof());
}

