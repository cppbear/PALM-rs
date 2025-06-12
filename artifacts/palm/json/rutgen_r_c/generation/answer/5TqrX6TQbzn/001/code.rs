// Answer 0

#[test]
fn test_eat_char() {
    struct TestRead {
        called: bool,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'a'))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'a'))
        }

        fn discard(&mut self) {
            self.called = true;
        }

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            0
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>, {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut test_read = TestRead { called: false };
    let mut deserializer = Deserializer { read: &mut test_read, scratch: vec![], remaining_depth: 0 };

    deserializer.eat_char();
    assert!(test_read.called, "Expected discard method to be called");
}

#[test]
fn test_eat_char_no_discard() {
    struct FailRead;

    impl<'de> Read<'de> for FailRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            unimplemented!();
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            unimplemented!();
        }

        fn discard(&mut self) {
            panic!("This should not be called!");
        }

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!();
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!();
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!();
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!();
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {
            unimplemented!();
        }

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>, {
            unimplemented!();
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!();
        }
    }

    let mut deserializer = Deserializer { read: &mut FailRead {}, scratch: vec![], remaining_depth: 0 };

    let result = std::panic::catch_unwind(|| {
        deserializer.eat_char();
    });

    assert!(result.is_err(), "Expected panic since discard should not be called");
}

