// Answer 0

#[test]
fn test_discard_with_valid_range() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'_> for TestReader {
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

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.position = self.data.len(); // Discard all data
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            todo!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }

        fn set_failed(&mut self, _: &mut bool) {
            todo!()
        }
    }

    let mut reader = TestReader { data: vec![1, 2, 3, 4, 5], position: 0 };
    reader.discard();
}

#[test]
fn test_discard_empty_reader() {
    struct EmptyReader;

    impl Read<'_> for EmptyReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            todo!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }

        fn set_failed(&mut self, _: &mut bool) {
            todo!()
        }
    }

    let mut reader = EmptyReader;
    reader.discard();
}

#[test]
#[should_panic]
fn test_discard_with_failed_state() {
    struct PanicReader {
        position: usize,
        failed: bool,
    }

    impl Read<'_> for PanicReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.failed {
                panic!("Reader in failed state");
            }
            Ok(Some(0))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.failed {
                panic!("Reader in failed state");
            }
            Ok(Some(0))
        }

        fn discard(&mut self) {
            self.failed = true; // Simulate setting the reader to a failed state
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.position }
        }

        fn byte_offset(&self) -> usize { 
            self.position 
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            todo!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            todo!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            todo!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            todo!()
        }

        fn set_failed(&mut self, failed: &mut bool) {
            *failed = self.failed;
        }
    }

    let mut reader = PanicReader { position: 0, failed: false };
    reader.discard(); // This will set the reader to failed state
    reader.discard(); // This should panic
}

