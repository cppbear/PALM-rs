// Answer 0

#[test]
fn test_display_index_out_of_bounds() {
    let error = GetDisjointMutError::IndexOutOfBounds;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "an index is out of bounds");
}

#[test]
fn test_display_overlapping_indices() {
    let error = GetDisjointMutError::OverlappingIndices;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "there were overlapping indices");
}

