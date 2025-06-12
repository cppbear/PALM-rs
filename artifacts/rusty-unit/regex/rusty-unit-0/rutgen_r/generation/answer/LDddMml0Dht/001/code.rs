// Answer 0

#[test]
fn test_bump_if_with_valid_prefix() {
    struct Parser {
        pattern: String,
        offset: usize,
    }

    impl Parser {
        fn new(pattern: String) -> Self {
            Self { pattern, offset: 0 }
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn bump(&mut self) {
            self.offset += 1;
        }

        fn bump_if(&mut self, prefix: &str) -> bool {
            if self.pattern()[self.offset()..].starts_with(prefix) {
                for _ in 0..prefix.chars().count() {
                    self.bump();
                }
                true
            } else {
                false
            }
        }
    }

    let mut parser = Parser::new("hello".to_string());
    assert!(parser.bump_if("he"));
    assert_eq!(parser.offset(), 2);
}

#[test]
fn test_bump_if_with_non_matching_prefix() {
    struct Parser {
        pattern: String,
        offset: usize,
    }

    impl Parser {
        fn new(pattern: String) -> Self {
            Self { pattern, offset: 0 }
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn bump(&mut self) {
            self.offset += 1;
        }

        fn bump_if(&mut self, prefix: &str) -> bool {
            if self.pattern()[self.offset()..].starts_with(prefix) {
                for _ in 0..prefix.chars().count() {
                    self.bump();
                }
                true
            } else {
                false
            }
        }
    }

    let mut parser = Parser::new("hello".to_string());
    assert!(!parser.bump_if("world"));
    assert_eq!(parser.offset(), 0);
}

#[test]
fn test_bump_if_with_empty_prefix() {
    struct Parser {
        pattern: String,
        offset: usize,
    }

    impl Parser {
        fn new(pattern: String) -> Self {
            Self { pattern, offset: 0 }
        }

        fn pattern(&self) -> &String {
            &self.pattern
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn bump(&mut self) {
            self.offset += 1;
        }

        fn bump_if(&mut self, prefix: &str) -> bool {
            if self.pattern()[self.offset()..].starts_with(prefix) {
                for _ in 0..prefix.chars().count() {
                    self.bump();
                }
                true
            } else {
                false
            }
        }
    }

    let mut parser = Parser::new("hello".to_string());
    assert!(parser.bump_if(""));
    assert_eq!(parser.offset(), 0);
}

