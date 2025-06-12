// Answer 0

#[test]
#[should_panic]
fn test_parse_unicode_escape_lone_leading_surrogate() {
    struct TestRead {
        state: usize,
    }

    impl TestRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            // Simulates the return of 0xDC00
            if self.state == 0 {
                self.state += 1;
                Ok(0xDC00)
            } else {
                Err(Error::custom("not expected"))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'u'))
        }

        fn discard(&mut self) {}
    }

    let mut read = TestRead { state: 0 };
    let mut scratch = Vec::new();
    let validate = true;

    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_valid() {
    struct TestRead {
        state: usize,
    }

    impl TestRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            // Simulates the return of 0xD800 followed by 0xDC00
            if self.state == 0 {
                self.state += 1;
                Ok(0xD800)
            } else if self.state == 1 {
                self.state += 1;
                Ok(0xDC00)
            } else {
                Err(Error::custom("not expected"))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'u'))
        }

        fn discard(&mut self) {}
    }

    let mut read = TestRead { state: 0 };
    let mut scratch = Vec::new();
    let validate = true;

    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_valid_no_surrogate() {
    struct TestRead {
        state: usize,
    }

    impl TestRead {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            // Simulates the return of a valid char outside of the surrogate range
            if self.state == 0 {
                self.state += 1;
                Ok(0x1000)
            } else {
                Err(Error::custom("not expected"))
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'\\'))
        }

        fn discard(&mut self) {}
    }

    let mut read = TestRead { state: 0 };
    let mut scratch = Vec::new();
    let validate = true;

    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

