// Answer 0

#[test]
fn test_parse_str_bytes_eof() {
    let slice = b"\"Hello, World!";
    let mut scratch = Vec::new();
    let mut reader = SliceRead::new(slice);

    // Move index to end of slice to trigger EOF condition
    reader.index = slice.len();

    let result = reader.parse_str_bytes(&mut scratch, true, |_, _| {
        Err(Error::from(ErrorCode::EofWhileParsingString))
    });

    assert!(result.is_err());
}

#[test]
fn test_parse_str_bytes_no_escape() {
    let slice = b"\"Hello, World!\"";
    let mut scratch = Vec::new();
    let mut reader = SliceRead::new(slice);
    
    let result = reader.parse_str_bytes(&mut scratch, false, |_, input| {
        assert_eq!(input, b"Hello, World!");
        Ok(&input)
    });

    assert!(result.is_ok());
    if let Ok(Reference::Borrowed(b)) = result {
        assert_eq!(b, b"Hello, World!");
    } else {
        panic!("Expected a borrowed reference");
    }
}

#[test]
fn test_parse_str_bytes_with_escape() {
    let slice = b"\"Hello, \\\"World!\\\"\"";
    let mut scratch = Vec::new();
    let mut reader = SliceRead::new(slice);
    
    let result = reader.parse_str_bytes(&mut scratch, true, |_, input| {
        assert_eq!(input, b"Hello, \"World!\"");
        Ok(&input)
    });

    assert!(result.is_ok());
    if let Ok(Reference::Copied(ref b)) = result {
        assert_eq!(b, b"Hello, \"World!\"");
    } else {
        panic!("Expected a copied reference");
    }
}

#[test]
fn test_parse_str_bytes_control_character() {
    let slice = b"\"Hello, \x01 World!\""; // has a control character
    let mut scratch = Vec::new();
    let mut reader = SliceRead::new(slice);
    
    let result = reader.parse_str_bytes(&mut scratch, true, |_, _| {
        Err(Error::from(ErrorCode::ControlCharacterWhileParsingString))
    });

    assert!(result.is_err());
}

