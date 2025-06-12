// Answer 0

#[derive(Debug)]
struct Literal {
    pat: Vec<u8>,
}

impl Literal {
    pub fn len(&self) -> usize {
        self.pat.len()
    }
    
    pub fn is_suffix(&self, text: &[u8]) -> bool {
        if text.len() < self.len() {
            return false;
        }
        text[text.len() - self.len()..] == *self.pat
    }
}

#[test]
fn test_is_suffix_true() {
    let literal = Literal { pat: b"end".to_vec() };
    assert!(literal.is_suffix(b"This is the end"));
}

#[test]
fn test_is_suffix_false() {
    let literal = Literal { pat: b"start".to_vec() };
    assert!(!literal.is_suffix(b"This is the end"));
}

#[test]
fn test_is_suffix_empty_string() {
    let literal = Literal { pat: b"".to_vec() };
    assert!(literal.is_suffix(b""));
}

#[test]
fn test_is_suffix_shorter_than_pattern() {
    let literal = Literal { pat: b"long".to_vec() };
    assert!(!literal.is_suffix(b"short"));
}

