// Answer 0

#[derive(Default)]
struct TestParser {
    input: Vec<u8>,
    position: usize,
}

impl TestParser {
    fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            Ok(ch)
        } else {
            Ok(0) // Simulating null
        }
    }

    fn peek_or_null(&self) -> Result<u8, &'static str> {
        if self.position < self.input.len() {
            Ok(self.input[self.position])
        } else {
            Ok(0) // Simulating null
        }
    }

    fn eat_char(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
        }
    }

    fn ignore_decimal(&mut self) -> Result<(), &'static str> {
        // Dummy implementation for testing.
        Ok(())
    }

    fn ignore_exponent(&mut self) -> Result<(), &'static str> {
        // Dummy implementation for testing.
        Ok(())
    }

    fn error(&self, _: ErrorCode) -> &'static str {
        "ERROR"
    }

    fn peek_error(&self, _: ErrorCode) -> &'static str {
        "PEEK_ERROR"
    }
}

#[derive(Debug)]
enum ErrorCode {
    InvalidNumber,
}

#[test]
fn test_ignore_integer_valid() {
    let mut parser = TestParser {
        input: vec![b'1', b'2', b'3', b'e', b'4'], // Valid integer followed by exponent
        ..Default::default()
    };
    assert_eq!(parser.ignore_integer(), Ok(()));
}

#[test]
fn test_ignore_integer_zero() {
    let mut parser = TestParser {
        input: vec![b'0'],
        ..Default::default()
    };
    assert_eq!(parser.ignore_integer(), Ok(()));
}

#[test]
fn test_ignore_integer_invalid_leading_zero() {
    let mut parser = TestParser {
        input: vec![b'0', b'1'], // Invalid case with leading zero
        ..Default::default()
    };
    assert_eq!(parser.ignore_integer(), Err("PEEK_ERROR"));
}

#[test]
fn test_ignore_integer_invalid_number() {
    let mut parser = TestParser {
        input: vec![b'a'], // Invalid character
        ..Default::default()
    };
    assert_eq!(parser.ignore_integer(), Err("ERROR"));
}

