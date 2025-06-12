// Answer 0

#[test]
fn test_decode_metadata_new_with_zero_decoded_bytes() {
    let decoded_bytes = 0;
    let padding_index: Option<usize> = None;
    let metadata = DecodeMetadata::new(decoded_bytes, padding_index);
    assert_eq!(metadata.decoded_len, decoded_bytes);
    assert_eq!(metadata.padding_offset, padding_index);
}

#[test]
fn test_decode_metadata_new_with_non_zero_decoded_bytes_and_none_padding() {
    let decoded_bytes = 5;
    let padding_index: Option<usize> = None;
    let metadata = DecodeMetadata::new(decoded_bytes, padding_index);
    assert_eq!(metadata.decoded_len, decoded_bytes);
    assert_eq!(metadata.padding_offset, padding_index);
}

#[test]
fn test_decode_metadata_new_with_non_zero_decoded_bytes_and_some_padding() {
    let decoded_bytes = 10;
    let padding_index: Option<usize> = Some(3);
    let metadata = DecodeMetadata::new(decoded_bytes, padding_index);
    assert_eq!(metadata.decoded_len, decoded_bytes);
    assert_eq!(metadata.padding_offset, padding_index);
}

#[test]
fn test_decode_metadata_new_with_large_decoded_bytes() {
    let decoded_bytes = usize::MAX;
    let padding_index: Option<usize> = Some(usize::MAX);
    let metadata = DecodeMetadata::new(decoded_bytes, padding_index);
    assert_eq!(metadata.decoded_len, decoded_bytes);
    assert_eq!(metadata.padding_offset, padding_index);
}

#[test]
#[should_panic]
fn test_decode_metadata_new_with_padding_index_exceeding_decoded_bytes() {
    let decoded_bytes = 2;
    let padding_index: Option<usize> = Some(3);
    let _metadata = DecodeMetadata::new(decoded_bytes, padding_index);
}

