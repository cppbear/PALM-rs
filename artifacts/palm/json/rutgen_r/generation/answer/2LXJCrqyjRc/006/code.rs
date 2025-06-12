// Answer 0

#[test]
fn test_do_deserialize_i128_valid_negative() {
    struct TestVisitor;
    
    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;
        
        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: String,
        position: usize,
    }

    impl TestDeserializer {
        fn new(input: &str) -> Self {
            Self { input: input.to_string(), position: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position..self.position + 1].as_bytes()[0];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }
        
        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            while self.position < self.input.len() {
                let byte = self.input[self.position..self.position + 1].as_bytes()[0];
                if byte.is_ascii_digit() || byte == b'-' {
                    buf.push(byte as char);
                    self.position += 1;
                } else {
                    break;
                }
            }
            Ok(())
        }

        fn do_deserialize_i128<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>,
        {
            let mut buf = String::new();

            match self.parse_whitespace()? {
                Some(b'-') => {
                    self.eat_char();
                    buf.push('-');
                }
                Some(_) => {}
                None => {
                    return Err(Error::EofWhileParsingValue);
                }
            }

            self.scan_integer128(&mut buf)?;

            let value = match buf.parse::<i128>() {
                Ok(int) => visitor.visit_i128(int),
                Err(_) => {
                    return Err(Error::NumberOutOfRange);
                }
            };

            value
        }
    }

    let mut deserializer = TestDeserializer::new("-12345");
    let result = deserializer.do_deserialize_i128(TestVisitor);
    assert_eq!(result, Ok(-12345));
}

#[test]
fn test_do_deserialize_i128_invalid_negative() {
    struct TestVisitor;
    
    impl<'any> de::Visitor<'any> for TestVisitor {
        type Value = i128;
        
        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct TestDeserializer {
        input: String,
        position: usize,
    }

    impl TestDeserializer {
        fn new(input: &str) -> Self {
            Self { input: input.to_string(), position: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position..self.position + 1].as_bytes()[0];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }
        
        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            buf.push('x'); // induces a parsing error
            Ok(())
        }

        fn do_deserialize_i128<'any, V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'any>,
        {
            let mut buf = String::new();
            match self.parse_whitespace()? {
                Some(b'-') => {
                    self.eat_char();
                    buf.push('-');
                }
                Some(_) => {}
                None => {
                    return Err(Error::EofWhileParsingValue);
                }
            }

            self.scan_integer128(&mut buf)?;

            match buf.parse::<i128>() {
                Ok(int) => visitor.visit_i128(int),
                Err(_) => {
                    return Err(Error::NumberOutOfRange);
                }
            }
        }
    }

    let mut deserializer = TestDeserializer::new("-x"); 
    let result = deserializer.do_deserialize_i128(TestVisitor);
    assert!(result.is_err());
}

