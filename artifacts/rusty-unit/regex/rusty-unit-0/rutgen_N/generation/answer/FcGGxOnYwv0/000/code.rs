// Answer 0

#[test]
fn test_is_lookaround_prefix_positive_lookahead() {
    struct Parser {
        input: &'static str,
        position: usize,
    }

    impl Parser {
        fn bump_if(&mut self, prefix: &str) -> bool {
            if self.input[self.position..].starts_with(prefix) {
                self.position += prefix.len();
                return true;
            }
            false
        }

        fn is_lookaround_prefix(&mut self) -> bool {
            self.bump_if("?=")
                || self.bump_if("?!")
                || self.bump_if("?<=")
                || self.bump_if("?<!")
        }
    }

    let mut parser = Parser {
        input: "?=",
        position: 0,
    };

    assert!(parser.is_lookaround_prefix());
}

#[test]
fn test_is_lookaround_prefix_negative() {
    struct Parser {
        input: &'static str,
        position: usize,
    }

    impl Parser {
        fn bump_if(&mut self, prefix: &str) -> bool {
            if self.input[self.position..].starts_with(prefix) {
                self.position += prefix.len();
                return true;
            }
            false
        }

        fn is_lookaround_prefix(&mut self) -> bool {
            self.bump_if("?=")
                || self.bump_if("?!")
                || self.bump_if("?<=")
                || self.bump_if("?<!")
        }
    }

    let mut parser = Parser {
        input: "invalid",
        position: 0,
    };

    assert!(!parser.is_lookaround_prefix());
}

#[test]
fn test_is_lookaround_prefix_lookbehind() {
    struct Parser {
        input: &'static str,
        position: usize,
    }

    impl Parser {
        fn bump_if(&mut self, prefix: &str) -> bool {
            if self.input[self.position..].starts_with(prefix) {
                self.position += prefix.len();
                return true;
            }
            false
        }

        fn is_lookaround_prefix(&mut self) -> bool {
            self.bump_if("?=")
                || self.bump_if("?!")
                || self.bump_if("?<=")
                || self.bump_if("?<!")
        }
    }

    let mut parser = Parser {
        input: "?<=",
        position: 0,
    };

    assert!(parser.is_lookaround_prefix());
}

