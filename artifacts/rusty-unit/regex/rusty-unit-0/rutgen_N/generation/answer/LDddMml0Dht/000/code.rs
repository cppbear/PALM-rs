// Answer 0

#[test]
fn test_bump_if_true() {
    struct TestParser {
        pattern: String,
        offset: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pattern: pattern.to_string(),
                offset: 0,
            }
        }

        fn pattern(&self) -> &str {
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

    let mut parser = TestParser::new("abcdef");
    assert!(parser.bump_if("abc"));
    assert_eq!(parser.offset(), 3);
}

#[test]
fn test_bump_if_false() {
    struct TestParser {
        pattern: String,
        offset: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pattern: pattern.to_string(),
                offset: 0,
            }
        }

        fn pattern(&self) -> &str {
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

    let mut parser = TestParser::new("abcdef");
    assert!(!parser.bump_if("xyz"));
    assert_eq!(parser.offset(), 0);
}

#[test]
fn test_bump_if_empty_prefix() {
    struct TestParser {
        pattern: String,
        offset: usize,
    }

    impl TestParser {
        fn new(pattern: &str) -> Self {
            TestParser {
                pattern: pattern.to_string(),
                offset: 0,
            }
        }

        fn pattern(&self) -> &str {
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

    let mut parser = TestParser::new("abcdef");
    assert!(parser.bump_if(""));
    assert_eq!(parser.offset(), 0);
}

