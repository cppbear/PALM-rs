// Answer 0

fn test_ignore_integer_zero_leading() {
    struct TestScanner {
        input: Vec<u8>,
        index: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(())
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn ignore_decimal(&mut self) -> Result<(), ()> {
            if self.peek_or_null().is_ok() {
                self.eat_char();
            }
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), ()> {
            if self.peek_or_null().is_ok() {
                self.eat_char();
            }
            Ok(())
        }
    }

    impl TestScanner {
        fn ignore_integer(&mut self) -> Result<(), ()> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(());
                    }
                }
                b'1'..=b'9' => {
                    while let Ok(ch) = self.peek_or_null() {
                        if ch.is_ascii_digit() {
                            self.eat_char();
                        } else {
                            break;
                        }
                    }
                }
                _ => return Err(()),
            }

            match self.peek_or_null()? {
                b'.' => self.ignore_decimal(),
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut scanner = TestScanner { input: vec![b'0', b'1', b'2', b'3'], index: 0 };
    assert_eq!(scanner.ignore_integer(), Err(()));
}

fn test_ignore_integer_valid() {
    struct TestScanner {
        input: Vec<u8>,
        index: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(())
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn ignore_decimal(&mut self) -> Result<(), ()> {
            if self.peek_or_null().is_ok() {
                self.eat_char();
            }
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), ()> {
            if self.peek_or_null().is_ok() {
                self.eat_char();
            }
            Ok(())
        }
    }

    impl TestScanner {
        fn ignore_integer(&mut self) -> Result<(), ()> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(());
                    }
                }
                b'1'..=b'9' => {
                    while let Ok(ch) = self.peek_or_null() {
                        if ch.is_ascii_digit() {
                            self.eat_char();
                        } else {
                            break;
                        }
                    }
                }
                _ => return Err(()),
            }

            match self.peek_or_null()? {
                b'.' => self.ignore_decimal(),
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut scanner = TestScanner { input: vec![b'3', b'2', b'1'], index: 0 };
    assert_eq!(scanner.ignore_integer(), Ok(()));
}

fn test_ignore_integer_invalid_character() {
    struct TestScanner {
        input: Vec<u8>,
        index: usize,
    }

    impl TestScanner {
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(())
            }
        }

        fn peek_or_null(&self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                Ok(self.input[self.index])
            } else {
                Err(())
            }
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn ignore_decimal(&mut self) -> Result<(), ()> {
            if self.peek_or_null().is_ok() {
                self.eat_char();
            }
            Ok(())
        }

        fn ignore_exponent(&mut self) -> Result<(), ()> {
            if self.peek_or_null().is_ok() {
                self.eat_char();
            }
            Ok(())
        }
    }

    impl TestScanner {
        fn ignore_integer(&mut self) -> Result<(), ()> {
            match self.next_char_or_null()? {
                b'0' => {
                    if let b'0'..=b'9' = self.peek_or_null()? {
                        return Err(());
                    }
                }
                b'1'..=b'9' => {
                    while let Ok(ch) = self.peek_or_null() {
                        if ch.is_ascii_digit() {
                            self.eat_char();
                        } else {
                            break;
                        }
                    }
                }
                _ => return Err(()),
            }

            match self.peek_or_null()? {
                b'.' => self.ignore_decimal(),
                b'e' | b'E' => self.ignore_exponent(),
                _ => Ok(()),
            }
        }
    }

    let mut scanner = TestScanner { input: vec![b'1', b'0', b'a'], index: 0 };
    assert_eq!(scanner.ignore_integer(), Err(()));
}

