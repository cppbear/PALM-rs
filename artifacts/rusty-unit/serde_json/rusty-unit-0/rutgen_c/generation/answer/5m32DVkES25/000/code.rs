// Answer 0

#[test]
fn test_next_or_eof_with_data() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        fn next(&mut self) -> Option<u8> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: (self.position + 1) as u32 }
        }
    }

    let mut reader = TestReader::new(vec![1, 2, 3]);
    assert_eq!(next_or_eof(&mut reader), Ok(1));
    assert_eq!(next_or_eof(&mut reader), Ok(2));
    assert_eq!(next_or_eof(&mut reader), Ok(3));
}

#[test]
#[should_panic]
fn test_next_or_eof_eof_error() {
    struct PanicReader {
        position: usize,
    }

    impl PanicReader {
        fn new() -> Self {
            Self { position: 0 }
        }
    }

    impl Read<'_> for PanicReader {
        fn next(&mut self) -> Option<u8> {
            None
        }

        fn position(&self) -> Position {
            Position { line: 1, column: (self.position + 1) as u32 }
        }
    }

    let mut reader = PanicReader::new();
    let _ = next_or_eof(&mut reader);
}

