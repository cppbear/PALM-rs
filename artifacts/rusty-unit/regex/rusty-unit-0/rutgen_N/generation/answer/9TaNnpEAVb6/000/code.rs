// Answer 0

#[derive(Default)]
struct PatternHolder {
    pattern: std::cell::RefCell<String>,
}

impl PatternHolder {
    fn new(pattern: &str) -> Self {
        Self {
            pattern: std::cell::RefCell::new(pattern.to_string()),
        }
    }
    
    fn pattern(&self) -> &str {
        self.pattern.borrow()
    }
}

#[test]
fn test_pattern_returns_correct_value() {
    let holder = PatternHolder::new("test_pattern");
    assert_eq!(holder.pattern(), "test_pattern");
}

#[test]
fn test_pattern_with_empty_string() {
    let holder = PatternHolder::new("");
    assert_eq!(holder.pattern(), "");
}

#[test]
fn test_pattern_with_whitespace() {
    let holder = PatternHolder::new("   ");
    assert_eq!(holder.pattern(), "   ");
}

