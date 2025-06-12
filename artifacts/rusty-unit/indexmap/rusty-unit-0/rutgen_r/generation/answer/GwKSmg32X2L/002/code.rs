// Answer 0

#[derive(Debug)]
enum GetDisjointMutError {
    IndexOutOfBounds,
    OverlappingIndices,
}

impl core::fmt::Display for GetDisjointMutError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            GetDisjointMutError::IndexOutOfBounds => "an index is out of bounds",
            GetDisjointMutError::OverlappingIndices => "there were overlapping indices",
        };

        core::fmt::Display::fmt(msg, f)
    }
}

#[test]
fn test_index_out_of_bounds() {
    let error = GetDisjointMutError::IndexOutOfBounds;
    let mut output = String::new();
    let result = error.fmt(&mut core::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "an index is out of bounds");
}

#[test]
fn test_overlapping_indices() {
    let error = GetDisjointMutError::OverlappingIndices;
    let mut output = String::new();
    let result = error.fmt(&mut core::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "there were overlapping indices");
}

