// Answer 0

struct PeekableIterator {
    data: Vec<u8>,
    position: usize,
}

impl PeekableIterator {
    fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }

    fn peek(&mut self) -> Result<Option<u8>, &'static str> {
        if self.position >= self.data.len() {
            return Err("EOF");
        }
        Ok(Some(self.data[self.position]))
    }

    fn eat_char(&mut self) {
        if self.position < self.data.len() {
            self.position += 1;
        }
    }

    fn parse_whitespace(&mut self) -> Result<Option<u8>, &'static str> {
        loop {
            match self.peek()? {
                Some(b' ' | b'\n' | b'\t' | b'\r') => {
                    self.eat_char();
                }
                other => {
                    return Ok(other);
                }
            }
        }
    }
}

#[test]
fn test_parse_whitespace_with_only_whitespace() {
    let mut iterator = PeekableIterator::new(vec![b' ', b'\n', b'\t']);
    assert_eq!(iterator.parse_whitespace().unwrap(), None);
}

#[test]
fn test_parse_whitespace_with_single_non_whitespace() {
    let mut iterator = PeekableIterator::new(vec![b' ', b'a']);
    assert_eq!(iterator.parse_whitespace().unwrap(), Some(b'a'));
}

#[test]
fn test_parse_whitespace_with_mixed_whitespace_and_non_whitespace() {
    let mut iterator = PeekableIterator::new(vec![b' ', b' ', b'\n', b'b', b'\t']);
    assert_eq!(iterator.parse_whitespace().unwrap(), Some(b'b'));
}

#[test]
fn test_parse_whitespace_empty_data() {
    let mut iterator = PeekableIterator::new(vec![]);
    assert_eq!(iterator.parse_whitespace().unwrap_err(), "EOF");
}

#[test]
fn test_parse_whitespace_only_tabs_and_newlines() {
    let mut iterator = PeekableIterator::new(vec![b'\t', b'\n', b'\r']);
    assert_eq!(iterator.parse_whitespace().unwrap(), None);
}

