// Answer 0

#[test]
fn test_fmt_index_out_of_bounds() {
    let error = GetDisjointMutError::IndexOutOfBounds;
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_fmt_overlapping_indices() {
    let error = GetDisjointMutError::OverlappingIndices;
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

