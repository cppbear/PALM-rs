// Answer 0

#[test]
fn test_new_with_padding_index() {
    struct TestStruct {
        decoded_len: usize,
        padding_offset: Option<usize>,
    }
    
    let result = TestStruct::new(10, Some(2));
    assert_eq!(result.decoded_len, 10);
    assert_eq!(result.padding_offset, Some(2));
}

#[test]
fn test_new_without_padding_index() {
    struct TestStruct {
        decoded_len: usize,
        padding_offset: Option<usize>,
    }

    let result = TestStruct::new(10, None);
    assert_eq!(result.decoded_len, 10);
    assert_eq!(result.padding_offset, None);
}

#[test]
fn test_new_with_zero_length() {
    struct TestStruct {
        decoded_len: usize,
        padding_offset: Option<usize>,
    }

    let result = TestStruct::new(0, Some(0));
    assert_eq!(result.decoded_len, 0);
    assert_eq!(result.padding_offset, Some(0));
}

