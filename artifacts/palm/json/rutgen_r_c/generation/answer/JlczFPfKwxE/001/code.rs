// Answer 0

#[test]
fn test_deserializer_new_with_slice_read() {
    struct SliceReader<'a> {
        data: &'a [u8],
        pos: usize,
    }

    impl<'a> read::Read<'a> for SliceReader<'a> {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) -> Result<()> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(())
            } else {
                Err(Error::new(ErrorCode::ExpectedChar))
            }
        }
    }

    let input: &[u8] = b"{}";
    let reader = SliceReader { data: input, pos: 0 };
    let deserializer = Deserializer::new(reader);

    assert_eq!(deserializer.remaining_depth, 128);
    assert!(deserializer.scratch.is_empty());
}

#[test]
fn test_deserializer_new_with_str_read() {
    struct StrReader<'a> {
        data: &'a str,
        pos: usize,
    }

    impl<'a> read::Read<'a> for StrReader<'a> {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data.as_bytes()[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) -> Result<()> {
            if self.pos < self.data.len() {
                self.pos += 1;
                Ok(())
            } else {
                Err(Error::new(ErrorCode::ExpectedChar))
            }
        }
    }

    let input = "{}";
    let reader = StrReader { data: input, pos: 0 };
    let deserializer = Deserializer::new(reader);

    assert_eq!(deserializer.remaining_depth, 128);
    assert!(deserializer.scratch.is_empty());
}

