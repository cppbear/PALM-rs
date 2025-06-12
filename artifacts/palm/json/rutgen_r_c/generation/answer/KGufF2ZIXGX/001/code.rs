// Answer 0

#[test]
fn test_discard_increments_index() {
    let mut data = SliceRead {
        slice: &[1, 2, 3],
        index: 0,
        #[cfg(feature = "raw_value")]
        raw_buffering_start_index: 0,
    };
    
    data.discard();
    assert_eq!(data.index, 1);
}

#[test]
fn test_discard_on_last_byte() {
    let mut data = SliceRead {
        slice: &[1, 2, 3],
        index: 2,
        #[cfg(feature = "raw_value")]
        raw_buffering_start_index: 0,
    };

    data.discard();
    assert_eq!(data.index, 3); // Should still increment even after the last byte
}

#[test]
fn test_discard_no_underflow() {
    let mut data = SliceRead {
        slice: &[1, 2, 3],
        index: 0,
        #[cfg(feature = "raw_value")]
        raw_buffering_start_index: 0,
    };

    data.discard();
    data.discard(); // Should work without panic
    assert_eq!(data.index, 2);
}

