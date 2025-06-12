// Answer 0

#[test]
fn test_parse_str_bytes_borrowed() {
    let json_data: &[u8] = b"\"Hello, world!\"";
    let mut slice_read = SliceRead::new(json_data);
    let mut scratch = Vec::new();

    let result: Result<Reference<'_, '_, str>> = slice_read.parse_str_bytes(&mut scratch, false, |_, bytes| {
        Ok(std::str::from_utf8(bytes).unwrap())
    });

    assert!(result.is_ok());
    if let Ok(Reference::Borrowed(borrowed)) = result {
        assert_eq!(borrowed, "Hello, world!");
    } else {
        panic!("Expected a borrowed reference.");
    }
}

#[test]
fn test_parse_str_bytes_copied() {
    let json_data: &[u8] = b"\"Goodbye, world!\"";
    let mut slice_read = SliceRead::new(json_data);
    let mut scratch = Vec::new();

    let result: Result<Reference<'_, '_, str>> = slice_read.parse_str_bytes(&mut scratch, false, |_, bytes| {
        Ok(std::str::from_utf8(bytes).unwrap())
    });

    assert!(result.is_ok());
    if let Ok(Reference::Copied(copied)) = result {
        assert_eq!(copied, "Goodbye, world!");
    } else {
        panic!("Expected a copied reference.");
    }
}

#[test]
#[should_panic]
fn test_parse_str_bytes_unexpected_control_character() {
    let json_data: &[u8] = b"\"Hello\x00World\""; // Contains a control character
    let mut slice_read = SliceRead::new(json_data);
    let mut scratch = Vec::new();

    let _ = slice_read.parse_str_bytes(&mut scratch, false, |_, _| {
        Ok("no-op")
    });
}

#[test]
fn test_parse_str_bytes_empty_string() {
    let json_data: &[u8] = b"\"\"";
    let mut slice_read = SliceRead::new(json_data);
    let mut scratch = Vec::new();

    let result: Result<Reference<'_, '_, str>> = slice_read.parse_str_bytes(&mut scratch, false, |_, bytes| {
        Ok(std::str::from_utf8(bytes).unwrap())
    });

    assert!(result.is_ok());
    if let Ok(Reference::Borrowed(borrowed)) = result {
        assert_eq!(borrowed, "");
    } else {
        panic!("Expected a borrowed reference.");
    }
}

#[test]
fn test_parse_str_bytes_escape_sequence() {
    let json_data: &[u8] = b"\"Hello \\\"world!\\\"\"";
    let mut slice_read = SliceRead::new(json_data);
    let mut scratch = Vec::new();

    let result: Result<Reference<'_, '_, str>> = slice_read.parse_str_bytes(&mut scratch, false, |_, bytes| {
        Ok(std::str::from_utf8(bytes).unwrap())
    });

    assert!(result.is_ok());
    if let Ok(Reference::Copied(copied)) = result {
        assert_eq!(copied, "Hello \"world!\"");
    } else {
        panic!("Expected a copied reference.");
    }
}

