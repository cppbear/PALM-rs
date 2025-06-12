// Answer 0

#[test]
fn test_ignore_exponent_positive() {
    struct TestStruct {
        input: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let c = self.input[self.index];
                self.index += 1;
                Ok(c)
            } else {
                Err(())
            }
        }

        fn ignore_exponent(&mut self) -> Result<(), ()> {
            self.eat_char();

            match self.peek_or_null()? {
                b'+' | b'-' => self.eat_char(),
                _ => {}
            }

            match self.next_char_or_null()? {
                b'0'..=b'9' => {}
                _ => {
                    return Err(());
                }
            }

            while let Ok(peek) = self.peek_or_null() {
                if !(b'0'..=b'9').contains(&peek) {
                    break;
                }
                self.eat_char();
            }

            Ok(())
        }
    }

    let mut test_instance = TestStruct {
        input: b"e+123".to_vec(),
        index: 0,
    };

    assert_eq!(test_instance.ignore_exponent(), Ok(()));
}

#[test]
fn test_ignore_exponent_negative() {
    struct TestStruct {
        input: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn eat_char(&mut self) {
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let c = self.input[self.index];
                self.index += 1;
                Ok(c)
            } else {
                Err(())
            }
        }

        fn ignore_exponent(&mut self) -> Result<(), ()> {
            self.eat_char();

            match self.peek_or_null()? {
                b'+' | b'-' => self.eat_char(),
                _ => {}
            }

            match self.next_char_or_null()? {
                b'0'..=b'9' => {}
                _ => {
                    return Err(());
                }
            }

            while let Ok(peek) = self.peek_or_null() {
                if !(b'0'..=b'9').contains(&peek) {
                    break;
                }
                self.eat_char();
            }

            Ok(())
        }
    }

    let mut test_instance = TestStruct {
        input: b"e+".to_vec(),
        index: 0,
    };

    assert_eq!(test_instance.ignore_exponent(), Err(()));
}

