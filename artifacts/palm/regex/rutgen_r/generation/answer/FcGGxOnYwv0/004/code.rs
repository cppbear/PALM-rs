// Answer 0

#[derive(Default)]
struct Parser {
    position: usize,
    input: &'static str,
}

impl Parser {
    fn bump_if(&mut self, prefix: &str) -> bool {
        if self.input[self.position..].starts_with(prefix) {
            self.position += prefix.len();
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

#[test]
fn test_is_lookaround_prefix_no_lookaround() {
    let mut parser = Parser {
        position: 0,
        input: "abc",
    };
    assert!(!parser.is_lookaround_prefix());
}

#[test]
fn test_is_lookaround_prefix_with_lookaround() {
    let mut parser = Parser {
        position: 0,
        input: "?=!abc",
    };
    assert!(parser.is_lookaround_prefix());
    assert_eq!(parser.position, 2);

    parser.position = 0;
    parser.input = "?!abc";
    assert!(parser.is_lookaround_prefix());
    assert_eq!(parser.position, 2);

    parser.position = 0;
    parser.input = "?<=abc";
    assert!(parser.is_lookaround_prefix());
    assert_eq!(parser.position, 3);

    parser.position = 0;
    parser.input = "?<!abc";
    assert!(parser.is_lookaround_prefix());
    assert_eq!(parser.position, 3);
}

