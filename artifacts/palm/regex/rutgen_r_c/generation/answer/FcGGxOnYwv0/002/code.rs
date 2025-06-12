// Answer 0

#[test]
fn test_is_lookaround_prefix_true_1() {
    struct DummyParser {
        pattern: String,
        offset: usize,
    }

    impl DummyParser {
        fn bump_if(&mut self, prefix: &str) -> bool {
            if self.pattern[self.offset..].starts_with(prefix) {
                self.offset += prefix.len();
                true
            } else {
                false
            }
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn is_lookaround_prefix(&mut self) -> bool {
            self.bump_if("?=")
                || self.bump_if("?!")
                || self.bump_if("?<=")
                || self.bump_if("?<!")
        }
    }

    let mut parser = DummyParser {
        pattern: String::from("?!something else"),
        offset: 0,
    };

    assert!(parser.is_lookaround_prefix());
}

#[test]
fn test_is_lookaround_prefix_false() {
    struct DummyParser {
        pattern: String,
        offset: usize,
    }

    impl DummyParser {
        fn bump_if(&mut self, prefix: &str) -> bool {
            if self.pattern[self.offset..].starts_with(prefix) {
                self.offset += prefix.len();
                true
            } else {
                false
            }
        }

        fn is_lookaround_prefix(&mut self) -> bool {
            self.bump_if("?=")
                || self.bump_if("?!")
                || self.bump_if("?<=")
                || self.bump_if("?<!")
        }
    }

    let mut parser = DummyParser {
        pattern: String::from("nothing at all"),
        offset: 0,
    };

    assert!(!parser.is_lookaround_prefix());
}

