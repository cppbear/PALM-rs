// Answer 0

#[derive(Debug)]
struct Pattern {
    value: String,
}

impl Pattern {
    fn new(value: &str) -> Self {
        Pattern { value: value.to_string() }
    }

    fn len(&self) -> usize {
        self.value.len()
    }
}

#[test]
fn test_len_empty_pattern() {
    let pattern = Pattern::new("");
    assert_eq!(pattern.len(), 0);
}

#[test]
fn test_len_non_empty_pattern() {
    let pattern = Pattern::new("abc");
    assert_eq!(pattern.len(), 3);
}

#[test]
fn test_len_whitespace_pattern() {
    let pattern = Pattern::new("   ");
    assert_eq!(pattern.len(), 3);
}

#[test]
fn test_len_special_characters() {
    let pattern = Pattern::new("!@#$%^&*()");
    assert_eq!(pattern.len(), 10);
}

