// Answer 0

#[test]
fn test_abort_std() {
    #[cfg(feature = "std")]
    {
        use std::panic;

        let result = panic::catch_unwind(|| {
            abort();
        });

        assert!(result.is_err());
    }
}

#[test]
#[should_panic(expected = "abort")]
fn test_abort_no_std() {
    #[cfg(not(feature = "std"))]
    {
        use std::panic;

        let result = panic::catch_unwind(|| {
            abort();
        });

        assert!(result.is_err());
    }
}

