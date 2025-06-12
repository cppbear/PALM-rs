// Answer 0

#[test]
fn test_parse_ident_success() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![Ok(b'a'), Ok(b'b'), Ok(b'c'), Ok(b'd')]),
        scratch: Vec::new(),
        remaining_depth: 5,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };
    let ident = [b'a', b'b', b'c'];
    let _ = deserializer.parse_ident(&ident);
}

#[test]
fn test_parse_ident_eof_error() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![Ok(b'a'), Ok(b'b'), Ok(b'c'), Err(ErrorCode::EofWhileParsingValue)]),
        scratch: Vec::new(),
        remaining_depth: 5,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };
    let ident = [b'a', b'b', b'c'];
    let _ = deserializer.parse_ident(&ident);
}

#[test]
fn test_parse_ident_expected_ident_error() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![Ok(b'e'), Ok(b'c'), Ok(b'f')]),
        scratch: Vec::new(),
        remaining_depth: 5,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };
    let ident = [b'd', b'e', b'f'];
    let _ = deserializer.parse_ident(&ident);
}

struct MockRead {
    input: Vec<Result<u8>>,
    position: usize,
}

impl MockRead {
    fn new(input: Vec<Result<u8>>) -> Self {
        MockRead { input, position: 0 }
    }
}

impl<'de> Read<'de> for MockRead {
    const should_early_return_if_failed: bool = false;

    fn next(&mut self) -> Result<Option<u8>> {
        if self.position >= self.input.len() {
            return Ok(None);
        }
        let result = self.input[self.position].clone();
        self.position += 1;
        Ok(result.ok())
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        if self.position >= self.input.len() {
            return Ok(None);
        }
        Ok(Some(self.input[self.position].as_ref().ok()?))
    }

    fn discard(&mut self) {}

    fn position(&self) -> Position {
        Position { line: 0, column: 0 }
    }

    fn peek_position(&self) -> Position {
        Position { line: 0, column: 0 }
    }

    fn byte_offset(&self) -> usize {
        self.position
    }

    fn parse_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, str>> {
        unimplemented!()
    }

    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
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
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn set_failed(&mut self, failed: &mut bool) {}
}

