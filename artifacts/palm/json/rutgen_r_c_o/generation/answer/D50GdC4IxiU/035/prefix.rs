// Answer 0

#[test]
fn test_parse_unicode_escape_case1() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(value as u16)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.index < self.data.len() {
                self.index += 1;
            }
        }
    }

    impl Read<'_> for MockReader {}

    let mut mock_reader = MockReader::new(vec![0xD800, b'\\', b'u']);
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut mock_reader, false, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_case2() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(value as u16)
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.index < self.data.len() {
                self.index += 1;
            }
        }
    }

    impl Read<'_> for MockReader {}

    let mut mock_reader = MockReader::new(vec![0xDBFF, b'\\', b'u']);
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut mock_reader, false, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_case3() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::from(ErrorCode::InvalidNumber))
        }

        fn peek(&self) -> Result<Option<u8>> {
            Ok(Some(b'\\'))
        }

        fn discard(&mut self) {
            if self.index < self.data.len() {
                self.index += 1;
            }
        }
    }

    impl Read<'_> for MockReader {}

    let mut mock_reader = MockReader::new(vec![0xD800, b'\\', b'u']);
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut mock_reader, false, &mut scratch);
}

