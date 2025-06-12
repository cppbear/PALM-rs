// Answer 0

#[test]
fn test_deserialize_number_valid_negative() {
    struct MockVisitor {
        value: i64,
    }
    
    impl<'a> de::Visitor<'a> for MockVisitor {
        type Value = i64;

        fn visit_i64(self, value: i64) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl MockDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn parse_whitespace(&mut self) -> core::result::Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                return Ok(Some(ch));
            }
            Err(())
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_integer(&mut self, _positive: bool) -> core::result::Result<i64, ()> {
            if self.index <= self.input.len() {
                let int_str = String::from_utf8_lossy(&self.input[self.index-1..self.index]).parse::<i64>().unwrap();
                self.index += 1; // Simulate consuming one character.
                return Ok(int_str);
            }
            Err(())
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            panic!("Unexpected EOF");
        }

        fn peek_invalid_type(&self, _visitor: &dyn de::Visitor<'_>) -> () {
            panic!("Invalid type encountered");
        }

        fn fix_position(&self, _err: ()) -> () {
            panic!("Error fixed position");
        }

        fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                _ => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_integer(false).and_then(|v| visitor.visit_i64(v))
                }
                b'0'..=b'9' => self.parse_integer(true).and_then(|v| visitor.visit_i64(v)),
                _ => Err(self.peek_invalid_type(&visitor)),
            };

            match value {
                Ok(v) => Ok(v),
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut deserializer = MockDeserializer::new(b"-1".to_vec());
    let visitor = MockVisitor { value: 0 };
    let result = deserializer.deserialize_number(visitor);
    assert_eq!(result, Ok(-1));
}

#[test]
fn test_deserialize_number_valid_positive() {
    struct MockVisitor {
        value: i64,
    }
    
    impl<'a> de::Visitor<'a> for MockVisitor {
        type Value = i64;

        fn visit_i64(self, value: i64) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl MockDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn parse_whitespace(&mut self) -> core::result::Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                return Ok(Some(ch));
            }
            Err(())
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_integer(&mut self, _positive: bool) -> core::result::Result<i64, ()> {
            if self.index <= self.input.len() {
                let int_str = String::from_utf8_lossy(&self.input[self.index-1..self.index]).parse::<i64>().unwrap();
                self.index += 1; // Simulate consuming one character.
                return Ok(int_str);
            }
            Err(())
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            panic!("Unexpected EOF");
        }

        fn peek_invalid_type(&self, _visitor: &dyn de::Visitor<'_>) -> () {
            panic!("Invalid type encountered");
        }

        fn fix_position(&self, _err: ()) -> () {
            panic!("Error fixed position");
        }

        fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                _ => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            let value = match peek {
                b'-' => {
                    self.eat_char();
                    self.parse_integer(false).and_then(|v| visitor.visit_i64(v))
                }
                b'0'..=b'9' => self.parse_integer(true).and_then(|v| visitor.visit_i64(v)),
                _ => Err(self.peek_invalid_type(&visitor)),
            };

            match value {
                Ok(v) => Ok(v),
                Err(err) => Err(self.fix_position(err)),
            }
        }
    }

    let mut deserializer = MockDeserializer::new(b"2".to_vec());
    let visitor = MockVisitor { value: 0 };
    let result = deserializer.deserialize_number(visitor);
    assert_eq!(result, Ok(2));
}

#[test]
#[should_panic]
fn test_deserialize_number_eof() {
    struct MockVisitor {
        value: i64,
    }

    impl<'a> de::Visitor<'a> for MockVisitor {
        type Value = i64;

        fn visit_i64(self, value: i64) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl MockDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn parse_whitespace(&mut self) -> core::result::Result<Option<u8>, ()> {
            return Err(());
        }

        fn eat_char(&mut self) {}

        fn parse_integer(&mut self, _positive: bool) -> core::result::Result<i64, ()> {
            Err(())
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            panic!("Unexpected EOF");
        }

        fn fix_position(&self, _err: ()) -> () {
            panic!("Error fixed position");
        }

        fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                _ => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            Err(())
        }
    }

    let mut deserializer = MockDeserializer::new(vec![]);
    let visitor = MockVisitor { value: 0 };
    deserializer.deserialize_number(visitor);
}

#[test]
#[should_panic]
fn test_deserialize_number_invalid_character() {
    struct MockVisitor {
        value: i64,
    }

    impl<'a> de::Visitor<'a> for MockVisitor {
        type Value = i64;

        fn visit_i64(self, value: i64) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockDeserializer {
        input: Vec<u8>,
        index: usize,
    }

    impl MockDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, index: 0 }
        }

        fn parse_whitespace(&mut self) -> core::result::Result<Option<u8>, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                return Ok(Some(ch));
            }
            Err(())
        }

        fn eat_char(&mut self) {
            self.index += 1;
        }

        fn parse_integer(&mut self, _positive: bool) -> core::result::Result<i64, ()> {
            Err(())
        }

        fn peek_error(&self, _code: ErrorCode) -> () {
            panic!("Unexpected EOF");
        }

        fn peek_invalid_type(&self, _visitor: &dyn de::Visitor<'_>) -> () {
            panic!("Invalid type encountered");
        }

        fn fix_position(&self, _err: ()) -> () {
            panic!("Error fixed position");
        }

        fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>,
        {
            let peek = match self.parse_whitespace() {
                Ok(Some(b)) => b,
                _ => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            match peek {
                b'a' => Err(self.peek_invalid_type(&visitor)),
                _ => Err(self.peek_invalid_type(&visitor)),
            }
        }
    }

    let mut deserializer = MockDeserializer::new(b"a".to_vec());
    let visitor = MockVisitor { value: 0 };
    deserializer.deserialize_number(visitor);
}

