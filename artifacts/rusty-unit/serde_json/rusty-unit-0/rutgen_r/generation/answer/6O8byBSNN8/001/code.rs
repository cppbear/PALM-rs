// Answer 0

#[test]
fn test_ignore_exponent_invalid_number_no_digit() {
    struct TestStruct {
        index: usize,
        data: Vec<u8>,
    }

    impl TestStruct {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(())
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.data.len() {
                let c = self.data[self.index];
                self.index += 1;
                Ok(c)
            } else {
                Err(())
            }
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Handle error
        }
    }

    enum ErrorCode {
        InvalidNumber,
    }

    let mut test_struct = TestStruct {
        index: 0,
        data: vec![b'e'], // Exponent indicator without a digit
    };

    let result = test_struct.ignore_exponent();
    assert!(result.is_err());
}

#[test]
fn test_ignore_exponent_invalid_number_non_digit() {
    struct TestStruct {
        index: usize,
        data: Vec<u8>,
    }

    impl TestStruct {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(())
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.data.len() {
                let c = self.data[self.index];
                self.index += 1;
                Ok(c)
            } else {
                Err(())
            }
        }

        fn error(&self, _code: ErrorCode) -> () {
            // Handle error
        }
    }

    enum ErrorCode {
        InvalidNumber,
    }

    let mut test_struct = TestStruct {
        index: 0,
        data: vec![b'e', b'+', b'a'], // Exponent with a non-digit character after
    };

    let result = test_struct.ignore_exponent();
    assert!(result.is_err());
}

