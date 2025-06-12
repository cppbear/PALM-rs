// Answer 0

#[test]
fn test_do_deserialize_i128_positive() {
    struct TestVisitor;
    
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = i128;

        fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct DummyReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for DummyReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut reader = DummyReader {
        input: b"12345".to_vec(),
        position: 0,
    };
    
    let mut deserializer = Deserializer {
        read: reader,
        scratch: vec![],
        remaining_depth: 10,
    };

    let result: Result<i128> = deserializer.do_deserialize_i128(TestVisitor);
    assert_eq!(result.unwrap(), 12345);
}

#[test]
fn test_do_deserialize_i128_negative() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = i128;

        fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    struct DummyReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for DummyReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut reader = DummyReader {
        input: b"-54321".to_vec(),
        position: 0,
    };
    
    let mut deserializer = Deserializer {
        read: reader,
        scratch: vec![],
        remaining_depth: 10,
    };

    let result: Result<i128> = deserializer.do_deserialize_i128(TestVisitor);
    assert_eq!(result.unwrap(), -54321);
}

#[test]
#[should_panic]
fn test_do_deserialize_i128_invalid() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = i128;

        fn visit_i128<E>(self, _value: i128) -> Result<Self::Value, E> {
            unimplemented!()
        }
    }

    struct DummyReader {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for DummyReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut reader = DummyReader {
        input: b"not_a_number".to_vec(),
        position: 0,
    };
    
    let mut deserializer = Deserializer {
        read: reader,
        scratch: vec![],
        remaining_depth: 10,
    };

    let _: Result<i128> = deserializer.do_deserialize_i128(TestVisitor);
}

