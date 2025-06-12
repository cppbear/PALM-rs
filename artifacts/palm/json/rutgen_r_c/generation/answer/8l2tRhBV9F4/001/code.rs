// Answer 0

#[test]
fn test_set_failed_success() {
    struct Delegate {
        failed: bool,
    }

    impl Delegate {
        fn set_failed(&mut self, failed: &mut bool) {
            self.failed = *failed;
        }
    }

    struct DummySliceRead<'a> {
        delegate: Delegate,
        index: usize,
    }

    impl<'a> SliceRead<'a> {
        fn new(slice: &'a [u8]) -> Self {
            Self {
                slice,
                index: 0,
            }
        }
    }

    impl<'a> Read<'a> for DummySliceRead<'a> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(self.delegate.failed as u8))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(self.delegate.failed as u8))
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>
        ) -> Result<Reference<'a, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>
        ) -> Result<Reference<'a, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn begin_raw_buffering(&mut self) {}

        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'a>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, failed: &mut bool) {
            self.delegate.set_failed(failed);
        }
    }

    let mut failed = false;
    let read = DummySliceRead {
        delegate: Delegate { failed },
        index: 0,
    };

    let mut read_ref = read; // mutable reference
    read_ref.set_failed(&mut failed);
    assert_eq!(read_ref.delegate.failed, false);

    failed = true;
    read_ref.set_failed(&mut failed);
    assert_eq!(read_ref.delegate.failed, true);
}

#[test]
#[should_panic]
fn test_set_failed_invalid_usage() {
    struct ErroneousDelegate;

    impl ErroneousDelegate {
        fn set_failed(&mut self, _failed: &mut bool) {
            panic!("Should not call set_failed");
        }
    }

    struct FaultySliceRead<'a> {
        delegate: ErroneousDelegate,
        index: usize,
    }

    impl<'a> Read<'a> for FaultySliceRead<'a> {
        const should_early_return_if_failed: bool = false;
        
        // Other method implementations omitted for brevity.

        fn set_failed(&mut self, failed: &mut bool) {
            self.delegate.set_failed(failed);
        }
    }

    let mut failed = true;
    let read = FaultySliceRead {
        delegate: ErroneousDelegate,
        index: 0,
    };

    let mut read_ref = read; // mutable reference
    read_ref.set_failed(&mut failed);
}

