// Answer 0

fn memchr(c: u8, text: &[u8]) -> Option<usize> {
    text.iter().position(|&x| x == c)
}

fn memchr2(c1: u8, c2: u8, text: &[u8]) -> Option<usize> {
    text.iter().position(|&x| x == c1 || x == c2)
}

fn memchr3(c1: u8, c2: u8, c3: u8, text: &[u8]) -> Option<usize> {
    text.iter().position(|&x| x == c1 || x == c2 || x == c3)
}

struct Literal {
    dense: Vec<u8>,
}

impl Literal {
    fn find(&self, text: &[u8]) -> Option<usize> {
        match self.dense.len() {
            0 => None,
            1 => memchr(self.dense[0], text),
            2 => memchr2(self.dense[0], self.dense[1], text),
            3 => memchr3(self.dense[0], self.dense[1], self.dense[2], text),
            _ => None,  // Handling for other cases if needed
        }
    }
}

#[test]
fn test_find_with_exact_match() {
    let literal = Literal { dense: vec![b'a', b'b', b'c'] };
    let text = b"hello abc world";
    assert_eq!(literal.find(text), Some(6));
}

#[test]
fn test_find_with_partial_match() {
    let literal = Literal { dense: vec![b'a', b'b', b'c'] };
    let text = b"hello ab world";
    assert_eq!(literal.find(text), Some(6));
}

#[test]
fn test_find_no_match() {
    let literal = Literal { dense: vec![b'a', b'b', b'c'] };
    let text = b"hello xyz world";
    assert_eq!(literal.find(text), None);
}

#[test]
fn test_find_empty_text() {
    let literal = Literal { dense: vec![b'a', b'b', b'c'] };
    let text: &[u8] = &[];
    assert_eq!(literal.find(text), None);
}

#[test]
fn test_find_exact_match_at_start() {
    let literal = Literal { dense: vec![b'a', b'b', b'c'] };
    let text = b"abc world";
    assert_eq!(literal.find(text), Some(0));
}

#[test]
fn test_find_multiple_matches() {
    let literal = Literal { dense: vec![b'a', b'b', b'c'] };
    let text = b"abc abc abc";
    assert_eq!(literal.find(text), Some(0)); // First match
}

