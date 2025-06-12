// Answer 0

#[test]
fn test_parse_str_bytes_with_valid_string() {
    struct JsonParser {
        index: usize,
        slice: &'static [u8],
    }

    let mut json = JsonParser { 
        index: 0, 
        slice: b"\"valid string\"" 
    };
    let mut scratch = vec![];

    let result = json.parse_str_bytes(&mut scratch, true, |_, data| {
        assert_eq!(data, b"valid string");
        Ok(&data[..])
    });

    assert!(result.is_ok());
}

#[test]
fn test_parse_str_bytes_with_escape() {
    struct JsonParser {
        index: usize,
        slice: &'static [u8],
    }

    let mut json = JsonParser { 
        index: 0, 
        slice: b"\"string with \\\" escape\"" 
    };
    let mut scratch = vec![];

    let result = json.parse_str_bytes(&mut scratch, true, |_, data| {
        assert_eq!(data, b"string with \\\" escape");
        Ok(&data[..])
    });

    assert!(result.is_ok());
}

#[test]
fn test_parse_str_bytes_with_control_character() {
    struct JsonParser {
        index: usize,
        slice: &'static [u8],
    }

    let mut json = JsonParser { 
        index: 0, 
        slice: b"\"invalid\x00character\"" 
    };
    let mut scratch = vec![];

    let result = json.parse_str_bytes(&mut scratch, true, |_, _| {
        panic!("This callback should not be executed");
    });

    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_parse_str_bytes_with_unescaped_backslash() {
    struct JsonParser {
        index: usize,
        slice: &'static [u8],
    }

    let mut json = JsonParser { 
        index: 0, 
        slice: b"\"string with an unescaped backslash \\" 
    };
    let mut scratch = vec![];

    json.parse_str_bytes(&mut scratch, true, |_, _| {
        panic!("Expected to panic due to unescaped backslash");
    });
}

