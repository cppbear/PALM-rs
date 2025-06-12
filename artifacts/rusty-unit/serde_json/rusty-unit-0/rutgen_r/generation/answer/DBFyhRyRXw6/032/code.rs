// Answer 0

#[test]
fn test_parse_exponent_valid() {
    struct DummyParser {
        index: usize,
        chars: Vec<u8>,
    }

    impl DummyParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            self.chars.get(self.index).copied().ok_or(())
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.chars.len() {
                let result = self.chars[self.index];
                self.eat_char();
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64, ()> {
            Ok(0.0) // Dummy implementation for testing
        }

        fn error(&self, _: ()) -> () {
            ()
        }
    }

    let mut parser = DummyParser { index: 0, chars: b"e+10".to_vec() };
    let result = parser.parse_exponent(true, 1, 0);
    assert_eq!(result, Ok(0.0));
}

#[test]
#[should_panic]
fn test_parse_exponent_invalid_character() {
    struct DummyParser {
        index: usize,
        chars: Vec<u8>,
    }

    impl DummyParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            self.chars.get(self.index).copied().ok_or(())
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.chars.len() {
                let result = self.chars[self.index];
                self.eat_char();
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64, ()> {
            Ok(0.0) // Dummy implementation for testing
        }

        fn error(&self, _: ()) -> () {
            ()
        }
    }

    let mut parser = DummyParser { index: 0, chars: b"e+a".to_vec() };
    parser.parse_exponent(true, 1, 0);
}

#[test]
fn test_parse_exponent_overflow() {
    struct DummyParser {
        index: usize,
        chars: Vec<u8>,
    }

    impl DummyParser {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            self.chars.get(self.index).copied().ok_or(())
        }

        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.chars.len() {
                let result = self.chars[self.index];
                self.eat_char();
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn f64_from_parts(&self, _positive: bool, _significand: u64, _exp: i32) -> Result<f64, ()> {
            Ok(0.0) // Dummy implementation for testing
        }

        fn error(&self, _: ()) -> () {
            ()
        }

        fn parse_exponent_overflow(&self, _: bool, _: bool, _: bool) -> Result<f64, ()> {
            Err(()) // Simulate overflow behavior
        }
    }

    let mut parser = DummyParser { index: 0, chars: b"e+9999999999".to_vec() };
    let result = parser.parse_exponent(true, 1, 0);
    assert_eq!(result, Err(()));
}

