// Answer 0

#[test]
fn test_discard_removes_peeked_byte() {
    struct MockRead {
        ch: Option<u8>,
    }

    impl MockRead {
        fn new() -> Self {
            MockRead { ch: Some(1) }
        }
    }

    #[cfg(feature = "raw_value")]
    impl Read<'static> for IoRead<MockRead> {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(self.ch.take())
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(self.ch)
        }

        #[cfg(feature = "raw_value")]
        fn discard(&mut self) {
            self.ch = None;
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

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
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
            V: Visitor<'static>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut mock_read = MockRead::new();
    let mut io_read = IoRead {
        iter: LineColIterator { iter: mock_read, line: 1, col: 1, start_of_line: 0 },
        ch: Some(2),
        raw_buffer: None,
    };

    io_read.discard();
    assert_eq!(io_read.ch, None);
}

