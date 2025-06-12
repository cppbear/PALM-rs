// Answer 0

#[test]
fn test_set_failed() {
    struct TestRead {
        failed: bool,
    }

    impl TestRead {
        fn new() -> Self {
            TestRead { failed: false }
        }
    }

    impl private::Sealed for TestRead {}

    impl Read<'static> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, failed: &mut bool) {
            self.failed = *failed;
        }
    }

    let mut test_reader = TestRead::new();
    let mut failed = true;

    test_reader.set_failed(&mut failed);
    assert!(test_reader.failed);
}

