// Answer 0

#[test]
fn test_parse_number_positive_u64() {
    struct DummyRead;

    impl<'de> Read<'de> for DummyRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'0')) // Simulating reading a digit '0'
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'0')) // Simulating peeking the same digit
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default() // Provide a default implementation
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unreachable!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unreachable!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unreachable!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unreachable!()
        }
    }

    let mut deserializer = Deserializer {
        read: DummyRead,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.parse_number(true, 42).unwrap();
    match result {
        ParserNumber::U64(value) => assert_eq!(value, 42),
        _ => panic!("Expected U64 variant"),
    }
}

#[test]
fn test_parse_number_negative_i64() {
    struct DummyRead;

    impl<'de> Read<'de> for DummyRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'-')) // Simulating reading a negative sign
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'0')) // Simulating peeking a digit '0'
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default() // Provide a default implementation
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unreachable!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unreachable!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unreachable!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unreachable!()
        }
    }

    let mut deserializer = Deserializer {
        read: DummyRead,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.parse_number(false, 42).unwrap();
    match result {
        ParserNumber::I64(value) => assert_eq!(value, -42),
        _ => panic!("Expected I64 variant"),
    }
}

#[test]
fn test_parse_number_float() {
    struct DummyRead;

    impl<'de> Read<'de> for DummyRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'.')) // Simulating reading a decimal point
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'1')) // Simulating peeking the next digit '1'
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default() // Provide a default implementation
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unreachable!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unreachable!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unreachable!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unreachable!()
        }
    }

    let mut deserializer = Deserializer {
        read: DummyRead,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let result = deserializer.parse_number(true, 42).unwrap();
    match result {
        ParserNumber::F64(value) => {
            assert!((value - 42.0).abs() < f64::EPSILON); // Check if float is as expected
        },
        _ => panic!("Expected F64 variant"),
    }
}

