// Answer 0

#[test]
fn test_decode_hex_escape_success() {
    struct TestReader<'a> {
        input: &'a [char],
        position: usize,
    }
    
    impl<'a> TestReader<'a> {
        fn next_or_eof(&mut self) -> Result<char> {
            if self.position < self.input.len() {
                let result = Ok(self.input[self.position]);
                self.position += 1;
                result
            } else {
                Err(ErrorCode::EndOfFile)
            }
        }
    }

    let mut reader = TestReader { 
        input: &['0', '1', 'a', 'f'], 
        position: 0 
    };
    
    let result = decode_hex_escape(&mut reader);
    assert_eq!(result, Ok(0x01AF));
}

#[test]
#[should_panic]
fn test_decode_hex_escape_invalid_input() {
    struct TestReader<'a> {
        input: &'a [char],
        position: usize,
    }
    
    impl<'a> TestReader<'a> {
        fn next_or_eof(&mut self) -> Result<char> {
            if self.position < self.input.len() {
                let result = Ok(self.input[self.position]);
                self.position += 1;
                result
            } else {
                Err(ErrorCode::EndOfFile)
            }
        }
    }
    
    let mut reader = TestReader { 
        input: &['g', 'h', 'i', 'j'], 
        position: 0 
    };

    let _ = decode_hex_escape(&mut reader); // Should trigger panic due to invalid hex digits
}

#[test]
fn test_decode_hex_escape_eof() {
    struct TestReader<'a> {
        input: &'a [char],
        position: usize,
    }
    
    impl<'a> TestReader<'a> {
        fn next_or_eof(&mut self) -> Result<char> {
            if self.position < self.input.len() {
                let result = Ok(self.input[self.position]);
                self.position += 1;
                result
            } else {
                Err(ErrorCode::EndOfFile)
            }
        }
    }

    let mut reader = TestReader { 
        input: &['0', '1'], 
        position: 0 
    };
    
    let result = decode_hex_escape(&mut reader);
    assert_eq!(result, Err(ErrorCode::InvalidEscape));
}

#[test]
fn test_decode_hex_escape_exceed_length() {
    struct TestReader<'a> {
        input: &'a [char],
        position: usize,
    }
    
    impl<'a> TestReader<'a> {
        fn next_or_eof(&mut self) -> Result<char> {
            if self.position < self.input.len() {
                let result = Ok(self.input[self.position]);
                self.position += 1;
                result
            } else {
                Err(ErrorCode::EndOfFile)
            }
        }
    }
    
    let mut reader = TestReader { 
        input: &[], 
        position: 0 
    };

    let result = decode_hex_escape(&mut reader);
    assert_eq!(result, Err(ErrorCode::InvalidEscape));
}

