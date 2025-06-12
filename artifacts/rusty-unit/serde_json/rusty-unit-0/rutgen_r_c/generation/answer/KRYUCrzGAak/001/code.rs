// Answer 0

#[test]
fn test_discard_function() {
    struct TestReader {
        discarded: bool,
    }

    impl TestReader {
        fn new() -> Self {
            TestReader { discarded: false }
        }
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {
            self.discarded = true;
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

        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader::new();
    reader.discard();

    assert!(reader.discarded, "Discard should set discarded to true");
}

#[test]
fn test_discard_multiple_calls() {
    struct MultiCallReader {
        call_count: usize,
    }

    impl MultiCallReader {
        fn new() -> Self {
            MultiCallReader { call_count: 0 }
        }
    }

    impl Read<'static> for MultiCallReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {
            self.call_count += 1;
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

        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MultiCallReader::new();
    for _ in 0..3 {
        reader.discard();
    }

    assert_eq!(reader.call_count, 3, "Discard should be called 3 times");
}

