// Answer 0

#[test]
fn test_peek_end_of_value_with_whitespace() {
    struct Dummy {
        de: DummyDecoder,
    }

    struct DummyDecoder {
        input: Vec<u8>,
        position: usize,
    }

    impl DummyDecoder {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn read(&mut self) -> &mut Self {
            self.position += 1;
            self
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position + 1 }
        }
    }

    struct Position {
        line: usize,
        column: usize,
    }

    impl Dummy {
        fn peek_end_of_value(&mut self) -> Result<(), ()> {
            match self.de.peek()? {
                Some(b' ' | b'\n' | b'\t' | b'\r' | b'"' | b'[' | b']' | b'{' | b'}' | b',' | b':')
                | None => Ok(()),
                Some(_) => {
                    let position = self.de.peek_position();
                    Err(())
                }
            }
        }
    }

    let mut decoder = DummyDecoder { input: vec![b' ', b'\n', b'\t', b'"', b'[', b']', b'{', b'}', b',', b':'], position: 0 };
    let mut dummy = Dummy { de: decoder };

    assert!(dummy.peek_end_of_value().is_ok());
}

#[test]
fn test_peek_end_of_value_with_non_whitespace() {
    struct Dummy {
        de: DummyDecoder,
    }

    struct DummyDecoder {
        input: Vec<u8>,
        position: usize,
    }

    impl DummyDecoder {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn read(&mut self) -> &mut Self {
            self.position += 1;
            self
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position + 1 }
        }
    }

    struct Position {
        line: usize,
        column: usize,
    }

    impl Dummy {
        fn peek_end_of_value(&mut self) -> Result<(), ()> {
            match self.de.peek()? {
                Some(b' ' | b'\n' | b'\t' | b'\r' | b'"' | b'[' | b']' | b'{' | b'}' | b',' | b':')
                | None => Ok(()),
                Some(_) => {
                    let position = self.de.peek_position();
                    Err(())
                }
            }
        }
    }

    let mut decoder = DummyDecoder { input: vec![b'a'], position: 0 };
    let mut dummy = Dummy { de: decoder };

    assert!(dummy.peek_end_of_value().is_err());
}

#[test]
fn test_peek_end_of_value_empty_input() {
    struct Dummy {
        de: DummyDecoder,
    }

    struct DummyDecoder {
        input: Vec<u8>,
        position: usize,
    }

    impl DummyDecoder {
        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn read(&mut self) -> &mut Self {
            self.position += 1;
            self
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position + 1 }
        }
    }

    struct Position {
        line: usize,
        column: usize,
    }

    impl Dummy {
        fn peek_end_of_value(&mut self) -> Result<(), ()> {
            match self.de.peek()? {
                Some(b' ' | b'\n' | b'\t' | b'\r' | b'"' | b'[' | b']' | b'{' | b'}' | b',' | b':')
                | None => Ok(()),
                Some(_) => {
                    let position = self.de.peek_position();
                    Err(())
                }
            }
        }
    }

    let mut decoder = DummyDecoder { input: vec![], position: 0 };
    let mut dummy = Dummy { de: decoder };

    assert!(dummy.peek_end_of_value().is_ok());
}

