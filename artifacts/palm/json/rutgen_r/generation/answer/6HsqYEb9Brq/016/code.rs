// Answer 0

fn test_ignore_integer() {
    struct TestStruct {
        next_char: Vec<u8>,
        peeked_char: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            if self.index < self.next_char.len() {
                let result = self.next_char[self.index];
                self.index += 1;
                Ok(result)
            } else {
                Err("EOF")
            }
        }

        fn peek_or_null(&self) -> Result<u8, &'static str> {
            if self.index < self.peeked_char.len() {
                Ok(self.peeked_char[self.index])
            } else {
                Err("EOF")
            }
        }

        fn eat_char(&mut self) {
            // Simulate eating a character by advancing the index
            self.index += 1;
        }

        fn error(&self, _: ErrorCode) -> &'static str {
            "Invalid Number Error"
        }

        fn peek_error(&self, _: ErrorCode) -> &'static str {
            "Peek Error"
        }

        fn ignore_decimal(&mut self) -> Result<(), &'static str> {
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), &'static str> {
            Ok(())
        }
    }

    #[derive(Debug)]
    enum ErrorCode {
        InvalidNumber,
    }

    // Test case: leading '0' followed by another digit
    {
        let mut test_struct = TestStruct {
            next_char: vec![b'0', b'1'],
            peeked_char: vec![b'0'],
            index: 0,
        };
        let result = test_struct.ignore_integer();
        assert_eq!(result, Err("Peek Error"));
    }

    // Test case: valid integer with leading non-zero digit
    {
        let mut test_struct = TestStruct {
            next_char: vec![b'1', b'2', b'3', b'4'],
            peeked_char: vec![b'.'],
            index: 0,
        };
        let result = test_struct.ignore_integer();
        assert_eq!(result, Ok(()));
    }

    // Test case: invalid character after a valid integer
    {
        let mut test_struct = TestStruct {
            next_char: vec![b'2'],
            peeked_char: vec![b'A'],
            index: 0,
        };
        let result = test_struct.ignore_integer();
        assert_eq!(result, Err("Invalid Number Error"));
    }

    // Test case: just '0' with valid peek
    {
        let mut test_struct = TestStruct {
            next_char: vec![b'0'],
            peeked_char: vec![b'.'],
            index: 0,
        };
        let result = test_struct.ignore_integer();
        assert_eq!(result, Ok(()));
    }

    // Test case: leading '0' and peek error
    {
        let mut test_struct = TestStruct {
            next_char: vec![b'0'],
            peeked_char: Vec::new(), // No peek available
            index: 0,
        };
        let result = test_struct.ignore_integer();
        assert_eq!(result, Err("EOF"));
    }
}

