// Answer 0

#[test]
fn test_ignore_exponent_valid() {
    struct TestStruct {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestStruct {
        fn eat_char(&mut self) -> u8 {
            let char_at_pos = self.input[self.pos];
            self.pos += 1;
            char_at_pos
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(0)
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.eat_char())
            } else {
                Ok(0)
            }
        }

        fn error(&self, _: ErrorCode) -> () {
            panic!("Error encountered");
        }
    }

    let mut test = TestStruct {
        input: b"E123".to_vec(),
        pos: 0,
    };

    let result = ignore_exponent(&mut test);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_ignore_exponent_invalid_no_digit() {
    struct TestStruct {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestStruct {
        fn eat_char(&mut self) -> u8 {
            let char_at_pos = self.input[self.pos];
            self.pos += 1;
            char_at_pos
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(0)
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.eat_char())
            } else {
                Ok(0)
            }
        }

        fn error(&self, _: ErrorCode) -> () {
            panic!("Error encountered");
        }
    }

    let mut test = TestStruct {
        input: b"E+".to_vec(),
        pos: 0,
    };

    let _ = ignore_exponent(&mut test);
}

#[test]
fn test_ignore_exponent_valid_multiple_digits() {
    struct TestStruct {
        input: Vec<u8>,
        pos: usize,
    }

    impl TestStruct {
        fn eat_char(&mut self) -> u8 {
            let char_at_pos = self.input[self.pos];
            self.pos += 1;
            char_at_pos
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.input[self.pos])
            } else {
                Ok(0)
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.pos < self.input.len() {
                Ok(self.eat_char())
            } else {
                Ok(0)
            }
        }

        fn error(&self, _: ErrorCode) -> () {
            panic!("Error encountered");
        }
    }

    let mut test = TestStruct {
        input: b"E+12345".to_vec(),
        pos: 0,
    };

    let result = ignore_exponent(&mut test);
    assert!(result.is_ok());
}

