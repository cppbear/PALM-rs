// Answer 0

#[test]
fn test_parse_str_bytes_empty_slice() {
    struct TestStruct {
        index: usize,
        slice: &'static [u8],
    }

    let mut scratch = Vec::<u8>::new();
    let mut test_instance = TestStruct {
        index: 0,
        slice: &[],
    };

    let result = parse_str_bytes(&mut test_instance, &mut scratch, false, |_, _| Ok(&""));
    assert!(result.is_err());
}

#[test]
fn test_parse_str_bytes_valid_string_no_escapes() {
    struct TestStruct {
        index: usize,
        slice: &'static [u8],
    }

    let mut scratch = Vec::<u8>::new();
    let mut test_instance = TestStruct {
        index: 0,
        slice: b"\"hello world\"",
    };

    let result = parse_str_bytes(&mut test_instance, &mut scratch, false, |_, borrowed| Ok(borrowed));
    assert!(result.is_ok());
    assert_eq!(test_instance.index, 14);
    assert!(scratch.is_empty());
}

#[test]
fn test_parse_str_bytes_valid_string_with_escapes() {
    struct TestStruct {
        index: usize,
        slice: &'static [u8],
    }

    let mut scratch = Vec::<u8>::new();
    let mut test_instance = TestStruct {
        index: 0,
        slice: b"\"hello \\\"world\\\"\"",
    };

    let result = parse_str_bytes(&mut test_instance, &mut scratch, true, |_, borrowed| Ok(borrowed));
    assert!(result.is_ok());
    assert_eq!(test_instance.index, 17);
    assert!(!scratch.is_empty());
    assert_eq!(scratch, b"hello \"world\"");
}

#[test]
#[should_panic]
fn test_parse_str_bytes_control_character() {
    struct TestStruct {
        index: usize,
        slice: &'static [u8],
    }

    let mut scratch = Vec::<u8>::new();
    let mut test_instance = TestStruct {
        index: 0,
        slice: b"\"hello \x00 world\"",
    };

    let _ = parse_str_bytes(&mut test_instance, &mut scratch, false, |_, _| Ok(&""));
}

