// Answer 0

fn test_deserialize_bool_true() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
        
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value> {
            Err(E::custom("Expected boolean value"))
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        position: usize,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, String> {
            // Simulating parsing whitespace
            if self.position < self.input.len() {
                self.position += 1; // Move forward
                Ok(Some(self.input[self.position - 1]))
            } else {
                Err("EOF".to_string())
            }
        }

        fn eat_char(&mut self) {
            // Just move the cursor forward
            self.position += 1;
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<(), String> {
            let ident_len = ident.len();
            let current_ident: Vec<u8> = self.input[self.position..self.position + ident_len].to_vec();
            if &current_ident == ident {
                self.position += ident_len; // Move position
                Ok(())
            } else {
                Err("Identifier mismatch".to_string())
            }
        }

        fn peek_error(&self, _: ErrorCode) -> String {
            "Error while peeking".to_string()
        }

        fn peek_invalid_type(&self, _: &dyn de::Visitor<'de>) -> String {
            "Invalid type".to_string()
        }

        fn fix_position(&self, err: String) -> String {
            err
        }
    }

    let mut deserializer = TestDeserializer {
        input: b"true".to_vec(),
        position: 0,
    };

    let result = deserializer.deserialize_bool(TestVisitor);
    assert_eq!(result, Ok(true));
}

fn test_deserialize_bool_false() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
        
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value> {
            Err(E::custom("Expected boolean value"))
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        position: usize,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, String> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
            } else {
                Err("EOF".to_string())
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<(), String> {
            let ident_len = ident.len();
            let current_ident: Vec<u8> = self.input[self.position..self.position + ident_len].to_vec();
            if &current_ident == ident {
                self.position += ident_len;
                Ok(())
            } else {
                Err("Identifier mismatch".to_string())
            }
        }

        fn peek_error(&self, _: ErrorCode) -> String {
            "Error while peeking".to_string()
        }

        fn peek_invalid_type(&self, _: &dyn de::Visitor<'de>) -> String {
            "Invalid type".to_string()
        }

        fn fix_position(&self, err: String) -> String {
            err
        }
    }

    let mut deserializer = TestDeserializer {
        input: b"false".to_vec(),
        position: 0,
    };

    let result = deserializer.deserialize_bool(TestVisitor);
    assert_eq!(result, Ok(false));
}

fn test_deserialize_bool_invalid() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            Ok(value)
        }
        
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value> {
            Err(E::custom("Expected boolean value"))
        }
    }

    struct TestDeserializer {
        input: Vec<u8>,
        position: usize,
    }

    impl TestDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, String> {
            if self.position < self.input.len() {
                self.position += 1;
                Ok(Some(self.input[self.position - 1]))
            } else {
                Err("EOF".to_string())
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }

        fn parse_ident(&mut self, ident: &[u8]) -> Result<(), String> {
            let ident_len = ident.len();
            if self.position + ident_len <= self.input.len() {
                let current_ident: Vec<u8> = self.input[self.position..self.position + ident_len].to_vec();
                if &current_ident == ident {
                    self.position += ident_len;
                    Ok(())
                } else {
                    Err("Identifier mismatch".to_string())
                }
            } else {
                Err("EOF".to_string())
            }
        }

        fn peek_error(&self, _: ErrorCode) -> String {
            "Error while peeking".to_string()
        }

        fn peek_invalid_type(&self, _: &dyn de::Visitor<'de>) -> String {
            "Invalid type".to_string()
        }

        fn fix_position(&self, err: String) -> String {
            err
        }
    }

    let mut deserializer = TestDeserializer {
        input: b"unknown".to_vec(),
        position: 0,
    };

    let result = deserializer.deserialize_bool(TestVisitor);
    assert!(result.is_err());
}

