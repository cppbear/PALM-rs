// Answer 0

fn fmt_overlapping_indices_test() {
    use core::fmt::{self, Write};

    struct GetDisjointMutError;

    impl GetDisjointMutError {
        const OverlappingIndices: GetDisjointMutError = GetDisjointMutError;
        const IndexOutOfBounds: GetDisjointMutError = GetDisjointMutError;

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let msg = match self {
                _ if std::ptr::eq(self, &Self::IndexOutOfBounds) => "an index is out of bounds",
                _ if std::ptr::eq(self, &Self::OverlappingIndices) => "there were overlapping indices",
                _ => unreachable!(),
            };
            fmt::Display::fmt(msg, f)
        }
    }

    let mut output = String::new();
    let error = GetDisjointMutError::OverlappingIndices;

    assert!(error.fmt(&mut output).is_ok());
    assert_eq!(output, "there were overlapping indices");
}

fn fmt_index_out_of_bounds_test() {
    use core::fmt::{self, Write};

    struct GetDisjointMutError;

    impl GetDisjointMutError {
        const OverlappingIndices: GetDisjointMutError = GetDisjointMutError;
        const IndexOutOfBounds: GetDisjointMutError = GetDisjointMutError;

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let msg = match self {
                _ if std::ptr::eq(self, &Self::IndexOutOfBounds) => "an index is out of bounds",
                _ if std::ptr::eq(self, &Self::OverlappingIndices) => "there were overlapping indices",
                _ => unreachable!(),
            };
            fmt::Display::fmt(msg, f)
        }
    }

    let mut output = String::new();
    let error = GetDisjointMutError::IndexOutOfBounds;

    assert!(error.fmt(&mut output).is_ok());
    assert_eq!(output, "an index is out of bounds");
}

