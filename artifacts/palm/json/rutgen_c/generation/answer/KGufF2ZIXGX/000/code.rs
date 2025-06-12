// Answer 0

#[test]
fn test_discard_increases_index() {
    let mut slice_read = SliceRead {
        slice: &[1, 2, 3, 4, 5],
        index: 0,
        #[cfg(feature = "raw_value")]
        raw_buffering_start_index: 0,
    };
    
    // Verify initial index
    assert_eq!(slice_read.index, 0);
    
    // Call discard
    slice_read.discard();
    
    // Verify index after discard
    assert_eq!(slice_read.index, 1);
}

#[test]
fn test_discard_multiple_times() {
    let mut slice_read = SliceRead {
        slice: &[1, 2, 3, 4, 5],
        index: 2,
        #[cfg(feature = "raw_value")]
        raw_buffering_start_index: 0,
    };
    
    // Call discard twice
    slice_read.discard();
    slice_read.discard();
    
    // Verify index after two discards
    assert_eq!(slice_read.index, 4);
}

