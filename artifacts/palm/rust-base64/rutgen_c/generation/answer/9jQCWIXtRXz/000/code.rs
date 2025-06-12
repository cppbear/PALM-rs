// Answer 0

#[test]
fn test_decode_metadata_new_with_zero_bytes() {
    let decoded_bytes = 0;
    let padding_index = None;
    let metadata = DecodeMetadata::new(decoded_bytes, padding_index);
    assert_eq!(metadata.decoded_len, 0);
    assert_eq!(metadata.padding_offset, None);
}

#[test]
fn test_decode_metadata_new_with_some_padding() {
    let decoded_bytes = 10;
    let padding_index = Some(5);
    let metadata = DecodeMetadata::new(decoded_bytes, padding_index);
    assert_eq!(metadata.decoded_len, 10);
    assert_eq!(metadata.padding_offset, Some(5));
}

#[test]
fn test_decode_metadata_new_with_full_padding() {
    let decoded_bytes = 15;
    let padding_index = Some(15);
    let metadata = DecodeMetadata::new(decoded_bytes, padding_index);
    assert_eq!(metadata.decoded_len, 15);
    assert_eq!(metadata.padding_offset, Some(15));
}

