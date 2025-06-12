// Answer 0

#[test]
fn test_discard() {
    struct TestSliceRead<'a> {
        slice: &'a [u8],
        index: usize,
    }

    impl<'a> TestSliceRead<'a> {
        fn discard(&mut self) {
            // Simulate the discard logic by simply incrementing the index
            self.index = cmp::min(self.index + 1, self.slice.len());
        }
    }

    struct TestStrRead<'a> {
        delegate: TestSliceRead<'a>,
    }

    impl<'a> Read<'a> for TestStrRead<'a> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(self.delegate.slice.get(self.delegate.index).copied())
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(self.delegate.slice.get(self.delegate.index).copied())
        }

        fn discard(&mut self) {
            self.delegate.discard();
        }

        fn position(&self) -> Position {
            Position {}
        }

        fn peek_position(&self) -> Position {
            Position {}
        }

        fn byte_offset(&self) -> usize {
            self.delegate.index
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'a, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn begin_raw_buffering(&mut self) {}

        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
        where
            V: Visitor<'a>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let data: &[u8] = b"hello";
    let mut slice_read = TestSliceRead { slice: data, index: 0 };
    let mut str_read = TestStrRead { delegate: slice_read };

    // Before discard
    assert_eq!(str_read.byte_offset(), 0);
    str_read.discard();
    // After discard
    assert_eq!(str_read.byte_offset(), 1);
}

