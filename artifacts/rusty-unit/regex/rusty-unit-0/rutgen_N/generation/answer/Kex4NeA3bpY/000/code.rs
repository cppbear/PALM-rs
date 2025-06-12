// Answer 0

#[test]
fn test_is_eof_at_end_of_pattern() {
    struct TestCursor {
        pattern: &'static str,
        offset: usize,
    }

    impl TestCursor {
        fn new(pattern: &'static str) -> Self {
            Self { pattern, offset: 0 }
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn pattern(&self) -> &'static str {
            self.pattern
        }

        fn bump(&mut self) {
            if self.offset < self.pattern.len() {
                self.offset += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.offset() == self.pattern().len()
        }
    }

    let mut cursor = TestCursor::new("abc");
    cursor.bump();
    cursor.bump();
    cursor.bump();
    
    assert!(cursor.is_eof());
}

#[test]
fn test_is_eof_not_at_end_of_pattern() {
    struct TestCursor {
        pattern: &'static str,
        offset: usize,
    }

    impl TestCursor {
        fn new(pattern: &'static str) -> Self {
            Self { pattern, offset: 0 }
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn pattern(&self) -> &'static str {
            self.pattern
        }

        fn bump(&mut self) {
            if self.offset < self.pattern.len() {
                self.offset += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.offset() == self.pattern().len()
        }
    }

    let mut cursor = TestCursor::new("abc");
    cursor.bump();
    cursor.bump();
    
    assert!(!cursor.is_eof());
}

#[test]
fn test_is_eof_on_empty_pattern() {
    struct TestCursor {
        pattern: &'static str,
        offset: usize,
    }

    impl TestCursor {
        fn new(pattern: &'static str) -> Self {
            Self { pattern, offset: 0 }
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn pattern(&self) -> &'static str {
            self.pattern
        }

        fn bump(&mut self) {
            if self.offset < self.pattern.len() {
                self.offset += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.offset() == self.pattern().len()
        }
    }

    let cursor = TestCursor::new("");
    
    assert!(cursor.is_eof());
}

