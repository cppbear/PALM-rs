// Answer 0

#[test]
#[should_panic(expected = "abort")]
fn test_abort_no_std() {
    #[cfg(not(feature = "std"))]
    {
        abort();
    }
}

#[test]
#[should_panic]
fn test_abort_std() {
    #[cfg(feature = "std")]
    {
        abort();
    }
}

