// Answer 0

#[test]
fn test_at_valid_index() {
    let input_data = b"hello";
    let byte_input = ByteInput {
        text: input_data,
        only_utf8: true,
    };
    let result = byte_input.at(1);
    assert_eq!(result.pos, 1);
    assert_eq!(result.len, 1);
    assert_eq!(result.byte, Some(b'e'));
}

#[test]
fn test_at_zero_index() {
    let input_data = b"world";
    let byte_input = ByteInput {
        text: input_data,
        only_utf8: true,
    };
    let result = byte_input.at(0);
    assert_eq!(result.pos, 0);
    assert_eq!(result.len, 1);
    assert_eq!(result.byte, Some(b'w'));
}

#[test]
fn test_at_last_index() {
    let input_data = b"test";
    let byte_input = ByteInput {
        text: input_data,
        only_utf8: true,
    };
    let result = byte_input.at(3);
    assert_eq!(result.pos, 3);
    assert_eq!(result.len, 1);
    assert_eq!(result.byte, Some(b't'));
}

#[test]
#[should_panic]
fn test_at_out_of_bounds_index() {
    let input_data = b"test";
    let byte_input = ByteInput {
        text: input_data,
        only_utf8: true,
    };
    let _ = byte_input.at(4);
}

#[test]
#[should_panic]
fn test_at_negative_index() {
    let input_data = b"test";
    let byte_input = ByteInput {
        text: input_data,
        only_utf8: true,
    };
    let _ = byte_input.at(usize::MAX); // using usize::MAX to force panic
}

