// Answer 0

#[test]
fn test_skip_to_escape_empty_string() {
    let mut obj = {
        struct TestStruct {
            index: usize,
            slice: Vec<u8>,
        }
        TestStruct { index: 0, slice: vec![] }
    };
    
    obj.skip_to_escape(false);
    
    assert_eq!(obj.index, 0);
}

#[test]
fn test_skip_to_escape_valid_characters() {
    let mut obj = {
        struct TestStruct {
            index: usize,
            slice: Vec<u8>,
        }
        TestStruct { index: 0, slice: b"abc\"def\\".to_vec() }
    };
    
    obj.skip_to_escape(false);
    
    assert_eq!(obj.index, 4); // Should skip to the quote
}

#[test]
fn test_skip_to_escape_forbidden_control_characters() {
    let mut obj = {
        struct TestStruct {
            index: usize,
            slice: Vec<u8>,
        }
        TestStruct { index: 0, slice: b"abc\0def\"ghi".to_vec() }
    };
    
    // should not skip control characters
    obj.skip_to_escape(true);
    
    assert_eq!(obj.index, 3); // Should stop at the null character
}

#[test]
fn test_skip_to_escape_consecutive_escapes() {
    let mut obj = {
        struct TestStruct {
            index: usize,
            slice: Vec<u8>,
        }
        TestStruct { index: 0, slice: b"abc\\u041b\\u0435def".to_vec() }
    };
    
    obj.skip_to_escape(false);
    
    assert_eq!(obj.index, 14); // Should skip to the end of the string
}

