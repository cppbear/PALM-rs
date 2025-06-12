// Answer 0

#[test]
fn test_decode_hex_escape_invalid_escape() {
    struct MockDecoder {
        inputs: Vec<u8>,
        index: usize,
    }

    impl MockDecoder {
        fn new(inputs: Vec<u8>) -> Self {
            Self { inputs, index: 0 }
        }

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.inputs.len() {
                let result = self.inputs[self.index];
                self.index += 1;
                Ok(result)
            } else {
                Err(ErrorCode::UnexpectedEof)
            }
        }
    }

    let mut decoder = MockDecoder::new(vec![b'G', b'H', b'I', b'J']); // Invalid hex escape (not 0-9 or a-f)

    let result = decoder.decode_hex_escape();

    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err, ErrorCode::InvalidEscape);
    }
}

#[test]
fn test_decode_hex_escape_eof_before_four_digits() {
    struct MockDecoder {
        inputs: Vec<u8>,
        index: usize,
    }

    impl MockDecoder {
        fn new(inputs: Vec<u8>) -> Self {
            Self { inputs, index: 0 }
        }

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.inputs.len() {
                let result = self.inputs[self.index];
                self.index += 1;
                Ok(result)
            } else {
                Err(ErrorCode::UnexpectedEof)
            }
        }
    }

    let mut decoder = MockDecoder::new(vec![b'1', b'2']); // Only two digits provided, expects four

    let result = decoder.decode_hex_escape();

    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err, ErrorCode::UnexpectedEof);
    }
}

