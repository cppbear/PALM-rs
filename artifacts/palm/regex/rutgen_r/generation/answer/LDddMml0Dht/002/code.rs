// Answer 0

#[test]
fn test_bump_if_successful() {
    struct Parser {
        pattern: String,
        offset: usize,
    }

    impl Parser {
        fn new(pattern: &str) -> Self {
            Parser {
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

    let mut parser = Parser::new("hello world");
    assert!(parser.bump_if("hello"));
    assert_eq!(parser.offset(), 5);
}

#[test]
fn test_bump_if_partial_prefix() {
    struct Parser {
        pattern: String,
        offset: usize,
    }

    impl Parser {
        fn new(pattern: &str) -> Self {
            Parser {
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

    let mut parser = Parser::new("hi there");
    assert!(!parser.bump_if("hello")); 
    assert_eq!(parser.offset(), 0);
}

#[test]
fn test_bump_if_empty_prefix() {
    struct Parser {
        pattern: String,
        offset: usize,
    }

    impl Parser {
        fn new(pattern: &str) -> Self {
            Parser {
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

    let mut parser = Parser::new("sample text");
    assert!(parser.bump_if("")); 
    assert_eq!(parser.offset(), 0); 
}

#[test]
fn test_bump_if_at_end_of_string() {
    struct Parser {
        pattern: String,
        offset: usize,
    }

    impl Parser {
        fn new(pattern: &str) -> Self {
            Parser {
                pattern: pattern.to_string(),
                offset: pattern.len(),
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

    let mut parser = Parser::new("finished");
    assert!(!parser.bump_if("finished")); 
    assert_eq!(parser.offset(), 8); 
}

