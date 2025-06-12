// Answer 0

#[test]
fn test_parse_str_bytes_with_backslash() {
    let mut scratch = Vec::new();
    let mut reader = IoRead {
        iter: LineColIterator {
            iter: core::slice::from_ref(&b'\\' as &u8).iter(),
            line: 1,
            col: 1,
            start_of_line: 0,
        },
        ch: None,
        raw_buffer: None,
    };
    let result = reader.parse_str_bytes::<_, _>(&mut scratch, false, |_, _| Ok(()));
}

#[test]
fn test_parse_str_bytes_with_quote() {
    let mut scratch = Vec::new();
    let mut reader = IoRead {
        iter: LineColIterator {
            iter: core::slice::from_ref(&b'"' as &u8).iter(),
            line: 1,
            col: 1,
            start_of_line: 0,
        },
        ch: None,
        raw_buffer: None,
    };
    let result = reader.parse_str_bytes::<_, _>(&mut scratch, false, |_, _| Ok(()));
}

#[test]
fn test_parse_str_bytes_with_null_character() {
    let mut scratch = Vec::new();
    let mut reader = IoRead {
        iter: LineColIterator {
            iter: core::slice::from_ref(&b'\x00' as &u8).iter(),
            line: 1,
            col: 1,
            start_of_line: 0,
        },
        ch: None,
        raw_buffer: None,
    };
    let result = reader.parse_str_bytes::<_, _>(&mut scratch, false, |_, _| Ok(()));
}

#[test]
fn test_parse_str_bytes_with_control_character() {
    let mut scratch = Vec::new();
    let mut reader = IoRead {
        iter: LineColIterator {
            iter: core::slice::from_ref(&b'\x1f' as &u8).iter(),
            line: 1,
            col: 1,
            start_of_line: 0,
        },
        ch: None,
        raw_buffer: None,
    };
    let result = reader.parse_str_bytes::<_, _>(&mut scratch, false, |_, _| Ok(()));
}

#[test]
#[should_panic]
fn test_parse_str_bytes_panic_on_invalid_character() {
    let mut scratch = Vec::new();
    let mut reader = IoRead {
        iter: LineColIterator {
            iter: core::slice::from_ref(&b'\x1e' as &u8).iter(),
            line: 1,
            col: 1,
            start_of_line: 0,
        },
        ch: None,
        raw_buffer: None,
    };
    let result = reader.parse_str_bytes::<_, _>(&mut scratch, true, |_, _| Ok(()));
}

