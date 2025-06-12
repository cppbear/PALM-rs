// Answer 0

fn test_ignore_exponent_valid() -> Result<()> {
    struct TestStruct {
        data: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            self.data.get(self.index).copied().ok_or(())
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            let val = self.peek_or_null()?;
            self.eat_char();
            Ok(val)
        }

        fn error(&self, _: ErrorCode) -> () {
            ()
        }
    }

    let mut test_instance = TestStruct {
        data: b"e+1".to_vec(),
        index: 0,
    };

    let result = test_instance.ignore_exponent();
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_ignore_exponent_invalid_no_digit() -> Result<()> {
    struct TestStruct {
        data: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            self.data.get(self.index).copied().ok_or(())
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            let val = self.peek_or_null()?;
            self.eat_char();
            Ok(val)
        }

        fn error(&self, _: ErrorCode) -> () {
            ()
        }
    }

    let mut test_instance = TestStruct {
        data: b"e+".to_vec(),
        index: 0,
    };

    let result = test_instance.ignore_exponent();
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_ignore_exponent_invalid_non_digit() -> Result<()> {
    struct TestStruct {
        data: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            self.data.get(self.index).copied().ok_or(())
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            let val = self.peek_or_null()?;
            self.eat_char();
            Ok(val)
        }

        fn error(&self, _: ErrorCode) -> () {
            ()
        }
    }

    let mut test_instance = TestStruct {
        data: b"e+z".to_vec(),
        index: 0,
    };

    let result = test_instance.ignore_exponent();
    assert!(result.is_err());
    Ok(())
}

