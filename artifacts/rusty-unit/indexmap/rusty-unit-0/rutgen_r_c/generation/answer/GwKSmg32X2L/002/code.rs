// Answer 0

#[test]
fn test_display_index_out_of_bounds() {
    let error = GetDisjointMutError::IndexOutOfBounds;
    let mut buffer = core::fmt::Formatter::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer.to_string(), "an index is out of bounds");
}

#[test]
fn test_display_overlapping_indices() {
    let error = GetDisjointMutError::OverlappingIndices;
    let mut buffer = core::fmt::Formatter::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer.to_string(), "there were overlapping indices");
}

