// Answer 0

#[test]
fn test_display_overlapping_indices() {
    let error = GetDisjointMutError::OverlappingIndices;
    let mut buffer = core::fmt::Formatter::new();
    error.fmt(&mut buffer);
}

#[test]
fn test_display_index_out_of_bounds() {
    let error = GetDisjointMutError::IndexOutOfBounds;
    let mut buffer = core::fmt::Formatter::new();
    error.fmt(&mut buffer);
}

