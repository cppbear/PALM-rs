// Answer 0

fn test_ignore_str_eof() {
    let slice = b"";
    let mut reader = SliceRead::new(slice);
    let result = reader.ignore_str();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.code(), ErrorCode::EofWhileParsingString);
    }
}

fn test_ignore_str_control_character() {
    let slice = b"test\x01"; // Control character (0x01)
    let mut reader = SliceRead::new(slice);
    let result = reader.ignore_str();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.code(), ErrorCode::ControlCharacterWhileParsingString);
    }
}

fn test_ignore_str_escaped() {
    let slice = b"escaped\\\""; // Should parse till the end successfully
    let mut reader = SliceRead::new(slice);
    let result = reader.ignore_str();
    assert!(result.is_ok());
}

fn test_ignore_str_unclosed() {
    let slice = b"unclosed"; // Missing closing quote
    let mut reader = SliceRead::new(slice);
    let result = reader.ignore_str();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.code(), ErrorCode::EofWhileParsingString);
    }
}

fn test_ignore_str_invalid_escape() {
    let slice = b"invalid\\x"; // Invalid escape sequence
    let mut reader = SliceRead::new(slice);
    let result = reader.ignore_str();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.code(), ErrorCode::ControlCharacterWhileParsingString);
    }
}

