// Answer 0

#[derive(Debug)]
struct Pattern {
    pat: String,
}

impl Pattern {
    pub fn new(pat: &str) -> Self {
        Pattern {
            pat: pat.to_owned(),
        }
    }
    
    pub fn len(&self) -> usize {
        self.pat.len()
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

#[test]
fn test_len_numeric_pattern() {
    let pattern = Pattern::new("1234567890");
    assert_eq!(pattern.len(), 10);
}

