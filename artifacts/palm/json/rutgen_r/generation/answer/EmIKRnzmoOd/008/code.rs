// Answer 0

#[derive(Debug)]
struct MockScanner {
    input: Vec<u8>,
    position: usize,
}

impl MockScanner {
    fn next_char_or_null(&mut self) -> Result<u8, ()> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            Ok(ch)
        } else {
            Ok(0) // Simulating null
        }
    }

    fn peek_or_null(&self) -> Result<u8, ()> {
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

    fn error(&self, _code: ErrorCode) -> () {
        ()
    }

    fn peek_error(&self, _code: ErrorCode) -> () {
        ()
    }
}

enum ErrorCode {
    InvalidNumber,
}

fn scan_integer128(sc: &mut MockScanner, buf: &mut String) -> Result<(), ()> {
    match sc.next_char_or_null()? {
        b'0' => {
            buf.push('0');
            match sc.peek_or_null()? {
                b'0'..=b'9' => return Err(()), // Invalid number
                _ => Ok(()),
            }
        }
        c @ b'1'..=b'9' => {
            buf.push(c as char);
            while let c @ b'0'..=b'9' = sc.peek_or_null()? {
                sc.eat_char();
                buf.push(c as char);
            }
            Ok(())
        }
        _ => Err(sc.error(ErrorCode::InvalidNumber)),
    }
}

#[test]
fn test_scan_integer128_valid() {
    let mut scanner = MockScanner {
        input: vec![b'1', b'2', b'3'],
        position: 0,
    };
    let mut buf = String::new();
    let result = scan_integer128(&mut scanner, &mut buf);
    assert_eq!(result, Ok(()));
    assert_eq!(buf, "123");
}

#[test]
fn test_scan_integer128_leading_zero_invalid() {
    let mut scanner = MockScanner {
        input: vec![b'0', b'1'],
        position: 0,
    };
    let mut buf = String::new();
    let result = scan_integer128(&mut scanner, &mut buf);
    assert_eq!(result, Err(())); // Should return error for leading zero
}

#[test]
fn test_scan_integer128_only_zero() {
    let mut scanner = MockScanner {
        input: vec![b'0'],
        position: 0,
    };
    let mut buf = String::new();
    let result = scan_integer128(&mut scanner, &mut buf);
    assert_eq!(result, Ok(()));
    assert_eq!(buf, "0"); // Should be valid, just '0'
}

#[test]
fn test_scan_integer128_empty() {
    let mut scanner = MockScanner {
        input: vec![],
        position: 0,
    };
    let mut buf = String::new();
    let result = scan_integer128(&mut scanner, &mut buf);
    assert!(result.is_err()); // Should handle empty input
}

#[test]
fn test_scan_integer128_multiple_digits() {
    let mut scanner = MockScanner {
        input: vec![b'4', b'5', b'6', b'7'],
        position: 0,
    };
    let mut buf = String::new();
    let result = scan_integer128(&mut scanner, &mut buf);
    assert_eq!(result, Ok(()));
    assert_eq!(buf, "4567");
}

#[test]
fn test_scan_integer128_invalid_character() {
    let mut scanner = MockScanner {
        input: vec![b'5', b'0', b'a'],
        position: 0,
    };
    let mut buf = String::new();
    let result = scan_integer128(&mut scanner, &mut buf);
    assert_eq!(result, Ok(()));
    assert_eq!(buf, "5"); // Should stop at invalid character
}

