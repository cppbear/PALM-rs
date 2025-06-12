// Answer 0

#[derive(Debug)]
struct Input {
    byte: Option<u8>,
}

impl Input {
    pub fn new(byte: Option<u8>) -> Self {
        Input { byte }
    }

    pub fn byte(&self) -> Option<u8> {
        self.byte
    }
}

#[test]
fn test_byte_some() {
    let input = Input::new(Some(5));
    assert_eq!(input.byte(), Some(5));
}

#[test]
fn test_byte_none() {
    let input = Input::new(None);
    assert_eq!(input.byte(), None);
}

