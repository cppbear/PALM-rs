// Answer 0

#[test]
fn test_deserialize_seq_empty() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(Vec::new())
        }
    }

    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(self.position, 1)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position, 1)
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
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MockReader { data: vec![b'[', b']'], position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 1 };

    let result: Result<Vec<u8>> = deserializer.deserialize_seq(MockVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Vec::<u8>::new());
}

#[test]
fn test_deserialize_seq_invalid() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(vec![1, 2, 3])
        }
    }

    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(self.position, 1)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position, 1)
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
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MockReader { data: vec![b'{'], position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 1 };

    let result: Result<Vec<u8>> = deserializer.deserialize_seq(MockVisitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_seq_non_empty() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, seq: V) -> Result<Self::Value>
        where
            V: de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            let mut seq = seq;
            while let Some(_) = seq.next_element::<u8>()? {
                values.push(0u8); // assume we push a dummy value
            }
            Ok(values)
        }
    }

    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                self.position += 1;
                Ok(Some(self.data[self.position - 1]))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(self.position, 1)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position, 1)
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
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MockReader { data: vec![b'[', b'0', b',', b'1', b']'], position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 1 };

    let result: Result<Vec<u8>> = deserializer.deserialize_seq(MockVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![0u8, 0u8]);
}

