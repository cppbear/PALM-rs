// Answer 0

#[test]
fn test_parse_str_raw_empty_input() {
    struct MockDelegate;

    impl Read<'static> for MockDelegate {
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
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Err(Error::new(ErrorCode::UnexpectedEnd))
        }
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            Ok(Reference::Borrowed(&[]))
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Err(Error::new(ErrorCode::InvalidHex))
        }
        fn set_failed(&mut self, failed: &mut bool) {}
    }

    struct StrRead<'a> {
        delegate: MockDelegate,
    }

    let mut scratch = Vec::new();
    let mut str_read = StrRead { delegate: MockDelegate };

    let result = str_read.parse_str_raw(&mut scratch);
    assert!(result.is_ok());
    if let Ok(reference) = result {
        assert!(matches!(reference, Reference::Borrowed(_)));
    }
}

#[test]
fn test_parse_str_raw_with_data() {
    struct MockDelegate;

    impl Read<'static> for MockDelegate {
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
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Ok(Reference::Borrowed("test"))
        }
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            scratch.extend_from_slice(b"raw_data");
            Ok(Reference::Borrowed(&scratch[..]))
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0x42)
        }
        fn set_failed(&mut self, failed: &mut bool) {}
    }

    struct StrRead<'a> {
        delegate: MockDelegate,
    }

    let mut scratch = Vec::new();
    let mut str_read = StrRead { delegate: MockDelegate };

    let result = str_read.parse_str_raw(&mut scratch);
    assert!(result.is_ok());
    if let Ok(reference) = result {
        assert!(matches!(reference, Reference::Borrowed(_)));
        assert_eq!(scratch.as_slice(), b"raw_data");
    }
}

#[test]
#[should_panic]
fn test_parse_str_raw_should_panic() {
    struct MockDelegate;

    impl Read<'static> for MockDelegate {
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
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            Err(Error::new(ErrorCode::UnexpectedEnd))
        }
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            panic!("Panic condition triggered");
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0x42)
        }
        fn set_failed(&mut self, failed: &mut bool) {}
    }

    struct StrRead<'a> {
        delegate: MockDelegate,
    }

    let mut scratch = Vec::new();
    let mut str_read = StrRead { delegate: MockDelegate };

    let _ = str_read.parse_str_raw(&mut scratch);
}

