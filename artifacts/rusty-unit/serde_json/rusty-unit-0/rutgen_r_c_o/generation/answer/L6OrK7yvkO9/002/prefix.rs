// Answer 0

#[test]
fn test_deserialize_bool_true() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn next_char(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            let id_len = ident.len();
            if self.position + id_len <= self.input.len() && &self.input[self.position..self.position + id_len] == ident {
                self.position += id_len;
                Ok(())
            } else {
                Err(Error::syntax(ErrorCode::ExpectedSomeIdent, 0, 0))
            }
        }

        fn discard(&mut self) {}
        
        fn parse_str(&self, _scratch: &mut Vec<u8>) -> Result<&str> {
            Ok("")
        }
    }

    let mut reader = TestReader {
        input: b"true".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let _result = deserializer.deserialize_bool(TestVisitor);
}

#[test]
fn test_deserialize_bool_false() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn next_char(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
            let id_len = ident.len();
            if self.position + id_len <= self.input.len() && &self.input[self.position..self.position + id_len] == ident {
                self.position += id_len;
                Ok(())
            } else {
                Err(Error::syntax(ErrorCode::ExpectedSomeIdent, 0, 0))
            }
        }

        fn discard(&mut self) {}
        
        fn parse_str(&self, _scratch: &mut Vec<u8>) -> Result<&str> {
            Ok("")
        }
    }

    let mut reader = TestReader {
        input: b"false".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let _result = deserializer.deserialize_bool(TestVisitor);
}

#[test]
fn test_deserialize_bool_invalid() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn next_char(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Err(Error::syntax(ErrorCode::ExpectedSomeIdent, 0, 0))
        }

        fn discard(&mut self) {}
        
        fn parse_str(&self, _scratch: &mut Vec<u8>) -> Result<&str> {
            Err(Error::syntax(ErrorCode::InvalidNumber, 0, 0))
        }
    }

    let mut reader = TestReader {
        input: b"notabool".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let _result = deserializer.deserialize_bool(TestVisitor);
}

#[test]
#[should_panic]
fn test_deserialize_bool_eof() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn next_char(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Ok(())
        }

        fn discard(&mut self) {}
        
        fn parse_str(&mut self, _scratch: &mut Vec<u8>) -> Result<&str> {
            Ok("")
        }
    }

    let mut reader = TestReader {
        input: Vec::new(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: reader,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let _result = deserializer.deserialize_bool(TestVisitor);
}

struct TestVisitor;

impl de::Visitor<'static> for TestVisitor {
    type Value = bool;

    fn visit_bool(self, value: bool) -> Result<Self::Value> {
        Ok(value)
    }
}

