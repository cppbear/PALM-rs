// Answer 0

#[test]
fn test_get_disjoint_mut_error_overlapping_indices_display() {
    let error = GetDisjointMutError::OverlappingIndices;
    let mut output = core::fmt::Formatter::new();
    
    assert_eq!(format!("{}", error), "there were overlapping indices");
}

#[test]
fn test_get_disjoint_mut_error_index_out_of_bounds_display() {
    let error = GetDisjointMutError::IndexOutOfBounds;
    let mut output = core::fmt::Formatter::new();
    
    assert_eq!(format!("{}", error), "an index is out of bounds");
}

