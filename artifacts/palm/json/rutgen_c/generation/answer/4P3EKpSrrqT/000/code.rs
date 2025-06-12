// Answer 0

#[test]
fn test_set_failed() {
    struct DummyReader {
        failed: bool,
    }

    impl private::Sealed for DummyReader {}

    impl Read<'_> for DummyReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            unimplemented!()
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            unimplemented!()
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            unimplemented!()
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
        }

        fn byte_offset(&self) -> usize {
            unimplemented!()
        }

        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'_>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, failed: &mut bool) {
            *failed = true;
        }
    }

    let mut reader = DummyReader { failed: false };
    let mut failure_flag = false;

    reader.set_failed(&mut failure_flag);
    assert!(failure_flag);
}

