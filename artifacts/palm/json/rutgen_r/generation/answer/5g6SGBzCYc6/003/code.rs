// Answer 0

fn test_parse_ident_valid_input() {
    struct TestParser {
        ident: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.ident.len() {
                let result = self.ident[self.index];
                self.index += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: ()) -> () {
            ()
        }
    }

    let mut parser = TestParser { ident: b"test".to_vec(), index: 0 };
    let result = parser.parse_ident(b"test");
    assert!(result.is_ok());
}

fn test_parse_ident_eof() {
    struct TestParser {
        ident: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.ident.len() {
                let result = self.ident[self.index];
                self.index += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: ()) -> () {
            ()
        }
    }

    let mut parser = TestParser { ident: b"test".to_vec(), index: 4 };
    let result = parser.parse_ident(b"test");
    assert!(result.is_err());
}

fn test_parse_ident_unexpected_char() {
    struct TestParser {
        ident: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.ident.len() {
                let result = self.ident[self.index];
                self.index += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: ()) -> () {
            ()
        }
    }

    let mut parser = TestParser { ident: b"testx".to_vec(), index: 0 };
    let result = parser.parse_ident(b"test");
    assert!(result.is_err());
}

fn test_parse_ident_empty_ident() {
    struct TestParser {
        ident: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.ident.len() {
                let result = self.ident[self.index];
                self.index += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: ()) -> () {
            ()
        }
    }

    let mut parser = TestParser { ident: b"".to_vec(), index: 0 };
    let result = parser.parse_ident(b"");
    assert!(result.is_ok());
}

fn test_parse_ident_partial_match() {
    struct TestParser {
        ident: Vec<u8>,
        index: usize,
    }

    impl TestParser {
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.ident.len() {
                let result = self.ident[self.index];
                self.index += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn error(&self, _code: ()) -> () {
            ()
        }
    }

    let mut parser = TestParser { ident: b"tes".to_vec(), index: 0 };
    let result = parser.parse_ident(b"test");
    assert!(result.is_err());
}

