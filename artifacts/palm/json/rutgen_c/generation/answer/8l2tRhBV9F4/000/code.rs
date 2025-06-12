// Answer 0

#[test]
fn test_set_failed() {
    struct MockDelegate {
        failed_flag: bool,
    }

    impl MockDelegate {
        fn set_failed(&mut self, failed: &mut bool) {
            self.failed_flag = *failed;
        }
    }

    struct MockStrRead<'a> {
        delegate: MockDelegate,
    }

    impl<'a> Read<'a> for MockStrRead<'a> {
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

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'a>,
        {
            todo!()
        }

        fn set_failed(&mut self, failed: &mut bool) {
            self.delegate.set_failed(failed);
        }
    }

    let mut failed = false;
    let mut str_read = MockStrRead {
        delegate: MockDelegate { failed_flag: false },
    };

    str_read.set_failed(&mut failed);
    assert_eq!(str_read.delegate.failed_flag, false);

    failed = true;
    str_read.set_failed(&mut failed);
    assert_eq!(str_read.delegate.failed_flag, true);
}

