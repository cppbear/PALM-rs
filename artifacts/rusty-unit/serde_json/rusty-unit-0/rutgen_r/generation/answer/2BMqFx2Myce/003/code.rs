// Answer 0

#[test]
fn test_parse_whitespace_first_non_whitespace() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            TestReader { input, position: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.position >= self.input.len() {
                return Ok(None);
            }
            Ok(Some(self.input[self.position]))
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
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

    let mut reader = TestReader::new(vec![b' ', b'\n', b'a']);
    assert_eq!(reader.parse_whitespace().unwrap(), Some(b'a'));

    let mut reader2 = TestReader::new(vec![b' ', b'\t', b'\r']);
    assert_eq!(reader2.parse_whitespace().unwrap(), None);

    let mut reader3 = TestReader::new(vec![b'\n', b'\t']);
    assert_eq!(reader3.parse_whitespace().unwrap(), None);

    let mut reader4 = TestReader::new(vec![b'a', b'\r']);
    assert_eq!(reader4.parse_whitespace().unwrap(), Some(b'a'));

    let mut reader5 = TestReader::new(vec![]);
    assert_eq!(reader5.parse_whitespace().unwrap(), None);
}

#[test]
fn test_parse_whitespace_with_err() {
    struct ErrReader;

    impl ErrReader {
        fn peek(&self) -> Result<Option<u8>, &'static str> {
            Err("EOF error")
        }
    }

    let err_reader = ErrReader;
    let result = err_reader.peek();
    assert!(result.is_err());
}

