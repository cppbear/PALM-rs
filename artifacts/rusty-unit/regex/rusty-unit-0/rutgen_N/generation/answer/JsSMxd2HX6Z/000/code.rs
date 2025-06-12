// Answer 0

#[derive(Debug)]
struct Input {
    text: Vec<u8>,
}

impl Input {
    fn new(text: Vec<u8>) -> Self {
        Input { text }
    }

    fn deref(&self) -> &[u8] {
        self.text.as_slice()
    }
}

#[test]
fn test_deref_non_empty() {
    let input = Input::new(vec![104, 101, 108, 108, 111]); // Represents "hello"
    assert_eq!(input.deref(), &[104, 101, 108, 108, 111]);
}

#[test]
fn test_deref_empty() {
    let input = Input::new(vec![]);
    assert_eq!(input.deref(), &[]);
}

#[test]
fn test_deref_single_element() {
    let input = Input::new(vec![65]); // Represents "A"
    assert_eq!(input.deref(), &[65]);
}

