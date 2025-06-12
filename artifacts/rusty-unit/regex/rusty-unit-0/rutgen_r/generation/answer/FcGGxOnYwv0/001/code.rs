// Answer 0

#[derive(Default)]
struct Parser {
    input: String,
    position: usize,
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
fn test_is_lookaround_prefix_true_equal() {
    let mut parser = Parser {
        input: String::from("?=someOtherText"),
        position: 0,
    };
    assert_eq!(parser.is_lookaround_prefix(), true);
}

#[test]
fn test_is_lookaround_prefix_true_not_equal() {
    let mut parser = Parser {
        input: String::from("?!otherText"),
        position: 0,
    };
    assert_eq!(parser.is_lookaround_prefix(), true);
}

#[test]
fn test_is_lookaround_prefix_true_positive_lookbehind() {
    let mut parser = Parser {
        input: String::from("?<="),
        position: 0,
    };
    assert_eq!(parser.is_lookaround_prefix(), true);
}

#[test]
fn test_is_lookaround_prefix_true_negative_lookbehind() {
    let mut parser = Parser {
        input: String::from("?<!suffix"),
        position: 0,
    };
    assert_eq!(parser.is_lookaround_prefix(), true);
}

#[test]
fn test_is_lookaround_prefix_false() {
    let mut parser = Parser {
        input: String::from("randomText"),
        position: 0,
    };
    assert_eq!(parser.is_lookaround_prefix(), false);
}

#[test]
fn test_is_lookaround_prefix_multiple_conditions() {
    let mut parser = Parser {
        input: String::from("?=???<!"), // first match is valid
        position: 0,
    };
    assert_eq!(parser.is_lookaround_prefix(), true);
}

