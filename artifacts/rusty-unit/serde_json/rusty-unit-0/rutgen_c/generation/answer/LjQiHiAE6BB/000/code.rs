// Answer 0

#[cfg(test)]
fn mock_reader(data: Vec<u8>, pos: usize) -> impl Read<'_> {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<R: ?Sized> Read<R> for MockReader {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn position(&self) -> Position {
            Position {
                line: 0,
                column: self.position as u32,
            }
        }
    }

    MockReader { data, position: pos }
}

#[test]
fn test_peek_or_eof_some() {
    let mut reader = mock_reader(vec![1, 2, 3], 0);
    let result = peek_or_eof(&mut reader);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn test_peek_or_eof_eof() {
    let mut reader = mock_reader(vec![], 0);
    let result = peek_or_eof(&mut reader);
    assert!(result.is_err());
}

