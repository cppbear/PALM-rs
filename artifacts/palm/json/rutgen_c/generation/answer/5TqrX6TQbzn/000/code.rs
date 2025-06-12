// Answer 0

#[test]
fn test_eat_char() {
    struct MockRead {
        discard_called: bool,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {
            self.discard_called = true;
        }

        fn position(&self) -> Position {
            unimplemented!()
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
        }

        fn byte_offset(&self) -> usize {
            unimplemented!()
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
    }

    let mut mock_read = MockRead {
        discard_called: false,
    };

    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    deserializer.eat_char();
    
    assert!(deserializer.read.discard_called);
}

