// Answer 0

#[test]
fn test_success_true() {
    struct MockVisitor;
    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockDeserializer {
        next_char_result: Result<Option<u8>, ()>,
        ident_result: Result<(), ()>,
    }

    impl MockDeserializer {
        fn eat_char(&self) {}
        
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            self.next_char_result.clone()
        }

        fn parse_ident(&self, _: &[u8]) -> Result<(), ()> {
            self.ident_result.clone()
        }

        fn peek_error(&self, _: ErrorCode) -> () { }
        
        fn fix_position(&self, err: ()) -> () { }
    }

    let mut deserializer = MockDeserializer {
        next_char_result: Ok(Some(b't')),
        ident_result: Ok(()),
    };

    let result = deserializer.deserialize_bool(MockVisitor);
    assert_eq!(result, Ok(true));
}

#[test]
fn test_success_false() {
    struct MockVisitor;
    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockDeserializer {
        next_char_result: Result<Option<u8>, ()>,
        ident_result: Result<(), ()>,
    }

    impl MockDeserializer {
        fn eat_char(&self) {}
        
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            self.next_char_result.clone()
        }

        fn parse_ident(&self, _: &[u8]) -> Result<(), ()> {
            self.ident_result.clone()
        }

        fn peek_error(&self, _: ErrorCode) -> () { }
        
        fn fix_position(&self, err: ()) -> () { }
    }

    let mut deserializer = MockDeserializer {
        next_char_result: Ok(Some(b'f')),
        ident_result: Ok(()),
    };

    let result = deserializer.deserialize_bool(MockVisitor);
    assert_eq!(result, Ok(false));
}

#[test]
fn test_error_unexpected_char() {
    struct MockVisitor;
    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, _: bool) -> Result<Self::Value> {
            Err(())
        }
    }

    struct MockDeserializer {
        next_char_result: Result<Option<u8>, ()>,
        ident_result: Result<(), ()>,
        parse_str_result: Result<(), ()>,
    }

    impl MockDeserializer {
        fn eat_char(&self) {}
        
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            self.next_char_result.clone()
        }

        fn parse_ident(&self, _: &[u8]) -> Result<(), ()> {
            self.ident_result.clone()
        }

        fn read(&self) -> Result<(), ()> {
            self.parse_str_result.clone()
        }

        fn peek_error(&self, _: ErrorCode) -> () { }
        
        fn fix_position(&self, err: ()) -> () { }
    }

    let mut deserializer = MockDeserializer {
        next_char_result: Ok(Some(b'x')),
        ident_result: Err(()),
        parse_str_result: Err(()),
    };

    let result = deserializer.deserialize_bool(MockVisitor);
    assert!(result.is_err());
}

#[test]
fn test_error_eof() {
    struct MockVisitor;
    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = bool;

        fn visit_bool(self, _: bool) -> Result<Self::Value> {
            Err(())
        }
    }

    struct MockDeserializer {
        next_char_result: Result<Option<u8>, ()>,
    }

    impl MockDeserializer {
        fn eat_char(&self) {}
        
        fn next_char(&mut self) -> Result<Option<u8>, ()> {
            self.next_char_result.clone()
        }

        fn peek_error(&self, _: ErrorCode) -> () { }
        
        fn fix_position(&self, err: ()) -> () { }
    }

    let mut deserializer = MockDeserializer {
        next_char_result: Err(()),
    };

    let result = deserializer.deserialize_bool(MockVisitor);
    assert!(result.is_err());
}

