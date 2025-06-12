// Answer 0

#[derive(Default)]
struct MockParser {
    chars: Vec<u8>,
    pos: usize,
}

impl MockParser {
    fn eat_char(&mut self) {
        self.pos += 1;
    }

    fn peek_or_null(&mut self) -> Result<u8, ()> {
        if self.pos < self.chars.len() {
            Ok(self.chars[self.pos])
        } else {
            Err(())
        }
    }

    fn next_char(&mut self) -> Result<Option<u8>, ()> {
        if self.pos < self.chars.len() {
            let ch = self.chars[self.pos];
            self.pos += 1;
            Ok(Some(ch))
        } else {
            Ok(None)
        }
    }

    fn error(&self, _code: ErrorCode) -> () {
        // Simulate error handling
    }

    fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64, ()> {
        // Simulate returning f64 from parts
        Ok(0.0)
    }

    fn parse_exponent_overflow(&self, _positive: bool, _zero_significand: bool, _positive_exp: bool) -> Result<f64, ()> {
        // Simulate overflow handling
        Err(())
    }
}

#[derive(Debug)]
enum ErrorCode {
    EofWhileParsingValue,
    InvalidNumber,
}

#[test]
fn test_parse_exponent_eof() {
    let mut parser = MockParser {
        chars: b"e".to_vec(),
        pos: 0,
    };
    let result = parser.parse_exponent(true, 1, 0);
    assert!(result.is_err());
}

#[test]
fn test_parse_exponent_invalid_number() {
    let mut parser = MockParser {
        chars: b"e-".to_vec(),
        pos: 0,
    };
    let result = parser.parse_exponent(true, 1, 0);
    assert!(result.is_err());
}

#[test]
fn test_parse_exponent_overflow() {
    let mut parser = MockParser {
        chars: b"e999999999999999999999999".to_vec(),
        pos: 0,
    };
    let result = parser.parse_exponent(true, 1, i32::MAX);
    assert!(result.is_err());
}

