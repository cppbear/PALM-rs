// Answer 0

#[test]
fn test_fmt_empty_slice() {
    struct EmptySlice;

    impl core::fmt::Display for EmptySlice {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Tried to create a `rand::distr::slice::Choose` with an empty slice")
        }
    }

    let empty_slice = EmptySlice;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", empty_slice);

    assert!(result.is_ok());
    assert_eq!(
        buffer,
        "Tried to create a `rand::distr::slice::Choose` with an empty slice"
    );
}

