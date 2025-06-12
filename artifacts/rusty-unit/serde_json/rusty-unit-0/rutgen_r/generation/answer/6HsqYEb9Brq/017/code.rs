// Answer 0

#[derive(Debug)]
struct TestStruct {
    current_char: Option<u8>,
    peek_char: Option<u8>,
}

impl TestStruct {
    fn next_char_or_null(&mut self) -> Result<u8, String> {
        self.current_char.map_or(Ok(0), |c| Ok(c)).or_else(|_| Err("No character".into()))
    }

    fn peek_or_null(&mut self) -> Result<u8, String> {
        self.peek_char.map_or(Ok(0), |c| Ok(c)).or_else(|_| Err("No character".into()))
    }

    fn eat_char(&mut self) {
        self.current_char = None; // simulate consuming a character
    }

    fn ignore_decimal(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn ignore_exponent(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn error(&self, _: ErrorCode) -> String {
        "Error".into()
    }

    fn peek_error(&self, _: ErrorCode) -> String {
        "Peek Error".into()
    }
}

#[derive(Debug)]
enum ErrorCode {
    InvalidNumber,
}

#[test]
fn test_ignore_integer_invalid_number_with_leading_zero() {
    let mut test_instance = TestStruct {
        current_char: Some(b'0'),
        peek_char: Some(b'0'),
    };
    
    let result = test_instance.ignore_integer();

    assert_eq!(result, Err(test_instance.peek_error(ErrorCode::InvalidNumber)), "Expected Err with InvalidNumber");
}

#[test]
fn test_ignore_integer_invalid_number_with_non_zero() {
    let mut test_instance = TestStruct {
        current_char: Some(b'0'),
        peek_char: Some(b'1'),
    };

    let result = test_instance.ignore_integer();

    assert_eq!(result, Ok(()), "Expected Ok for valid number");
}

#[test]
fn test_ignore_integer_with_malformed_number() {
    let mut test_instance = TestStruct {
        current_char: Some(b'1'),
        peek_char: Some(b'9'),
    };

    let result = test_instance.ignore_integer();

    assert_eq!(result, Ok(()), "Expected Ok for valid number");
}

#[test]
fn test_ignore_integer_with_exponent() {
    let mut test_instance = TestStruct {
        current_char: Some(b'1'),
        peek_char: Some(b'e'),
    };

    let result = test_instance.ignore_integer();

    assert_eq!(result, Ok(()), "Expected Ok for valid number");
}

#[test]
fn test_ignore_integer_with_decimal() {
    let mut test_instance = TestStruct {
        current_char: Some(b'1'),
        peek_char: Some(b'.'),
    };

    let result = test_instance.ignore_integer();

    assert_eq!(result, Ok(()), "Expected Ok for valid number");
}

