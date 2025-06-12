// Answer 0

#[test]
fn test_is_end_when_char_is_none() {
    let input = InputAt {
        pos: 0,
        c: Char(u32::MAX), // This will make c.is_none() true
        byte: None, // This makes byte.is_none() true
        len: 0,
    };
    assert!(input.is_end());
}

#[test]
fn test_is_end_when_char_is_none_and_byte_is_some() {
    let input = InputAt {
        pos: 1,
        c: Char(u32::MAX), // This will make c.is_none() true
        byte: Some(255), // This makes byte.is_none() false
        len: 1,
    };
    assert!(!input.is_end());
}

#[test]
fn test_is_end_when_char_is_some_and_byte_is_none() {
    let input = InputAt {
        pos: 2,
        c: Char(97), // This will make c.is_none() false (97 is 'a' in UTF-8)
        byte: None, // This makes byte.is_none() true
        len: 1,
    };
    assert!(!input.is_end());
}

#[test]
fn test_is_end_when_char_and_byte_are_both_some() {
    let input = InputAt {
        pos: 3,
        c: Char(98), // This will make c.is_none() false (98 is 'b' in UTF-8)
        byte: Some(128), // This makes byte.is_none() false 
        len: 1,
    };
    assert!(!input.is_end());
}

#[test]
fn test_is_end_with_minimal_input() {
    let input = InputAt {
        pos: 0,
        c: Char(u32::MAX), // This will make c.is_none() true
        byte: None, // This makes byte.is.none() true
        len: 0,
    };
    assert!(input.is_end());
}

