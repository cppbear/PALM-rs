// Answer 0

#[test]
fn test_byte_offset_zero_index() {
    let read_instance = SliceRead {
        slice: &[1, 2, 3],
        index: 0,
        #[cfg(feature = "raw_value")]
        raw_buffering_start_index: 0,
    };
    assert_eq!(read_instance.byte_offset(), 0);
}

#[test]
fn test_byte_offset_positive_index() {
    let read_instance = SliceRead {
        slice: &[1, 2, 3],
        index: 5,
        #[cfg(feature = "raw_value")]
        raw_buffering_start_index: 0,
    };
    assert_eq!(read_instance.byte_offset(), 5);
}

#[test]
fn test_byte_offset_at_slice_end() {
    let read_instance = SliceRead {
        slice: &[1, 2, 3],
        index: 3,
        #[cfg(feature = "raw_value")]
        raw_buffering_start_index: 0,
    };
    assert_eq!(read_instance.byte_offset(), 3);
}

#[test]
fn test_byte_offset_exceeding_slice_length() {
    let read_instance = SliceRead {
        slice: &[1, 2, 3],
        index: 10,
        #[cfg(feature = "raw_value")]
        raw_buffering_start_index: 0,
    };
    assert_eq!(read_instance.byte_offset(), 10);
}

