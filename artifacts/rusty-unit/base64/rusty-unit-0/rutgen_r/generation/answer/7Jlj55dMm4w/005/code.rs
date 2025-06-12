// Answer 0

#[test]
fn test_encoded_len_no_rem_no_padding() {
    let bytes_len = 0; // Test for 0 bytes
    let padding = false;
    assert_eq!(encoded_len(bytes_len, padding), Some(0));

    let bytes_len = 3; // Test for a complete chunk
    let padding = false;
    assert_eq!(encoded_len(bytes_len, padding), Some(4));

    let bytes_len = 6; // Test for two complete chunks
    let padding = false;
    assert_eq!(encoded_len(bytes_len, padding), Some(8));

    let bytes_len = 9; // Test for three complete chunks
    let padding = false;
    assert_eq!(encoded_len(bytes_len, padding), Some(12));
}

#[test]
fn test_encoded_len_no_rem_with_padding() {
    let bytes_len = 0; // Test for 0 bytes
    let padding = true;
    assert_eq!(encoded_len(bytes_len, padding), Some(0));

    let bytes_len = 3; // Test for a complete chunk
    let padding = true;
    assert_eq!(encoded_len(bytes_len, padding), Some(4));

    let bytes_len = 6; // Test for two complete chunks
    let padding = true;
    assert_eq!(encoded_len(bytes_len, padding), Some(8));

    let bytes_len = 9; // Test for three complete chunks
    let padding = true;
    assert_eq!(encoded_len(bytes_len, padding), Some(12));
}

