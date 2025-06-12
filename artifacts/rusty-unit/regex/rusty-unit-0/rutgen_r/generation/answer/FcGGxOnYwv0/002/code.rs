// Answer 0

#[test]
fn test_is_lookaround_prefix_true() {
    struct Parser {
        input: String,
        position: usize,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                position: 0,
            }
        }

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

    let mut parser = Parser::new("?!abc");
    assert_eq!(parser.is_lookaround_prefix(), true);
}

#[test]
fn test_is_lookaround_prefix_false() {
    struct Parser {
        input: String,
        position: usize,
    }

    impl Parser {
        fn new(input: &str) -> Self {
            Self {
                input: input.to_string(),
                position: 0,
            }
        }

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

    let mut parser = Parser::new("abc");
    assert_eq!(parser.is_lookaround_prefix(), false);
}

