// Answer 0

#[derive(Default)]
struct Parser {
    state: usize,
}

impl Parser {
    fn bump_if(&mut self, test: &str) -> bool {
        match test {
            "?<=" => {
                self.state += 1; // Represents a successful match for this case
                true
            }
            // Simulating false conditions for other tests based on constraints
            "?=" | "?!" | "?<!" => false,
            _ => false,
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
fn test_lookaround_prefix_valid() {
    let mut parser = Parser::default();
    assert_eq!(parser.is_lookaround_prefix(), true);
}

#[test]
fn test_lookaround_prefix_invalid() {
    let mut parser = Parser::default();
    // This condition should only yield false for our specified constraints
    assert_eq!(parser.is_lookaround_prefix(), true);
}

#[test]
fn test_lookaround_prefix_no_match() {
    let mut parser = Parser::default();
    parser.state = 0; // Ensuring parser is at the initial state
    assert_eq!(parser.is_lookaround_prefix(), false); // Test when all bump_if return false
}

