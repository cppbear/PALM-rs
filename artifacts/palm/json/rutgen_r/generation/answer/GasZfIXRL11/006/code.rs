// Answer 0

fn test_peek_invalid_type_sequence() {
    struct MockExpected;
    
    impl Expected for MockExpected {}

    struct MockDeserializer {
        data: Vec<u8>,
        position: usize,
        scratch: Vec<u8>,
    }

    impl MockDeserializer {
        fn peek_or_null(&mut self) -> Option<u8> {
            self.data.get(self.position).copied()
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), Error> {
            Ok(())
        }

        fn parse_any_number(&mut self, _: bool) -> Result<MockNumber, Error> {
            Err(Error)
        }

        fn fix_position(&mut self, err: Error) -> Error {
            err
        }

        fn peek_error(&mut self, _: ErrorCode) -> Error {
            Error
        }

        fn peek_invalid_type(&mut self, exp: &dyn Expected) -> Error {
            let err = match self.peek_or_null().unwrap_or(b'\x00') {
                b'n' => {
                    self.eat_char();
                    if let Err(err) = self.parse_ident(b"ull") {
                        return err;
                    }
                    de::Error::invalid_type(Unexpected::Unit, exp)
                }
                b't' => {
                    self.eat_char();
                    if let Err(err) = self.parse_ident(b"rue") {
                        return err;
                    }
                    de::Error::invalid_type(Unexpected::Bool(true), exp)
                }
                b'f' => {
                    self.eat_char();
                    if let Err(err) = self.parse_ident(b"alse") {
                        return err;
                    }
                    de::Error::invalid_type(Unexpected::Bool(false), exp)
                }
                b'-' => {
                    self.eat_char();
                    match self.parse_any_number(false) {
                        Ok(n) => n.invalid_type(exp),
                        Err(err) => return err,
                    }
                }
                b'0'..=b'9' => match self.parse_any_number(true) {
                    Ok(n) => n.invalid_type(exp),
                    Err(err) => return err,
                },
                b'"' => {
                    self.eat_char();
                    self.scratch.clear();
                    match self.read.parse_str(&mut self.scratch) {
                        Ok(s) => de::Error::invalid_type(Unexpected::Str(&s), exp),
                        Err(err) => return err,
                    }
                }
                b'[' => de::Error::invalid_type(Unexpected::Seq, exp),
                b'{' => de::Error::invalid_type(Unexpected::Map, exp),
                _ => self.peek_error(ErrorCode::ExpectedSomeValue),
            };

            self.fix_position(err)
        }
    }

    struct MockNumber;

    impl MockNumber {
        fn invalid_type(self, _: &dyn Expected) -> Error {
            Error
        }
    }

    let mut deserializer = MockDeserializer {
        data: vec![b'['],
        position: 0,
        scratch: Vec::new(),
    };
    let exp = MockExpected;

    let result = deserializer.peek_invalid_type(&exp);
    // Here we would normally check result against expected Error type
}

fn test_peek_invalid_type_map() {
    struct MockExpected;

    impl Expected for MockExpected {}

    struct MockDeserializer {
        data: Vec<u8>,
        position: usize,
        scratch: Vec<u8>,
    }

    impl MockDeserializer {
        fn peek_or_null(&mut self) -> Option<u8> {
            self.data.get(self.position).copied()
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn parse_ident(&mut self, _: &[u8]) -> Result<(), Error> {
            Ok(())
        }

        fn parse_any_number(&mut self, _: bool) -> Result<MockNumber, Error> {
            Err(Error)
        }

        fn fix_position(&mut self, err: Error) -> Error {
            err
        }

        fn peek_error(&mut self, _: ErrorCode) -> Error {
            Error
        }

        fn peek_invalid_type(&mut self, exp: &dyn Expected) -> Error {
            let err = match self.peek_or_null().unwrap_or(b'\x00') {
                b'n' => {
                    self.eat_char();
                    if let Err(err) = self.parse_ident(b"ull") {
                        return err;
                    }
                    de::Error::invalid_type(Unexpected::Unit, exp)
                }
                b't' => {
                    self.eat_char();
                    if let Err(err) = self.parse_ident(b"rue") {
                        return err;
                    }
                    de::Error::invalid_type(Unexpected::Bool(true), exp)
                }
                b'f' => {
                    self.eat_char();
                    if let Err(err) = self.parse_ident(b"alse") {
                        return err;
                    }
                    de::Error::invalid_type(Unexpected::Bool(false), exp)
                }
                b'-' => {
                    self.eat_char();
                    match self.parse_any_number(false) {
                        Ok(n) => n.invalid_type(exp),
                        Err(err) => return err,
                    }
                }
                b'0'..=b'9' => match self.parse_any_number(true) {
                    Ok(n) => n.invalid_type(exp),
                    Err(err) => return err,
                },
                b'"' => {
                    self.eat_char();
                    self.scratch.clear();
                    match self.read.parse_str(&mut self.scratch) {
                        Ok(s) => de::Error::invalid_type(Unexpected::Str(&s), exp),
                        Err(err) => return err,
                    }
                }
                b'[' => de::Error::invalid_type(Unexpected::Seq, exp),
                b'{' => de::Error::invalid_type(Unexpected::Map, exp),
                _ => self.peek_error(ErrorCode::ExpectedSomeValue),
            };

            self.fix_position(err)
        }
    }

    struct MockNumber;

    impl MockNumber {
        fn invalid_type(self, _: &dyn Expected) -> Error {
            Error
        }
    }

    let mut deserializer = MockDeserializer {
        data: vec![b'{'],
        position: 0,
        scratch: Vec::new(),
    };
    let exp = MockExpected;

    let result = deserializer.peek_invalid_type(&exp);
    // Here we would normally check result against expected Error type
}

