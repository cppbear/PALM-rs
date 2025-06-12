// Answer 0

#[test]
fn test_next_char_success() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> { Ok(self.data.get(self.position).copied()) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::from(self.position) }
        fn peek_position(&self) -> Position { Position::from(self.position) }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unreachable!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unreachable!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unreachable!() }
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value> where V: de::Visitor<'static> { unreachable!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let data = vec![104, 101, 108, 108, 111]; // Corresponds to "hello"
    let mock_read = MockRead { data, position: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: Vec::new(), remaining_depth: 0 };

    assert_eq!(deserializer.next_char().unwrap(), Some(104)); // 'h'
    assert_eq!(deserializer.next_char().unwrap(), Some(101)); // 'e'
    assert_eq!(deserializer.next_char().unwrap(), Some(108)); // 'l'
    assert_eq!(deserializer.next_char().unwrap(), Some(108)); // 'l'
    assert_eq!(deserializer.next_char().unwrap(), Some(111)); // 'o'
    assert_eq!(deserializer.next_char().unwrap(), None); // End of data
}

#[test]
fn test_next_char_empty() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::from(self.position) }
        fn peek_position(&self) -> Position { Position::from(self.position) }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unreachable!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unreachable!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unreachable!() }
        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}
        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value> where V: de::Visitor<'static> { unreachable!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mock_read = MockRead { data: vec![], position: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: Vec::new(), remaining_depth: 0 };

    assert_eq!(deserializer.next_char().unwrap(), None); // No characters to read
}

