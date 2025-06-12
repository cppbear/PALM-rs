// Answer 0

#[derive(Debug)]
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

#[test]
fn test_bump_if_prefix_not_found() {
    let mut parser = Parser::new("abcde");
    // Set offset to 3 so the current parser position is at 'd'
    parser.offset = 3;
    // Trying to find prefix "xyz" in "de" should return false
    assert_eq!(parser.bump_if("xyz"), false);
    // Ensure the offset remains unchanged
    assert_eq!(parser.offset(), 3);
}

#[test]
fn test_bump_if_empty_prefix() {
    let mut parser = Parser::new("abcde");
    parser.offset = 0; 
    // An empty prefix should always return true and bump the offset
    assert_eq!(parser.bump_if(""), true);
    assert_eq!(parser.offset(), 0); // offset should not change while looking for empty prefix
}

#[test]
fn test_bump_if_prefix_at_end() {
    let mut parser = Parser::new("abcde");
    parser.offset = 2; // Set offset to point at 'c'
    // Attempting to bump with the prefix "de", should return false since current position is 'c'
    assert_eq!(parser.bump_if("de"), false);
    assert_eq!(parser.offset(), 2); // offset should remain unchanged
}

