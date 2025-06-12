// Answer 0

#[test]
fn test_new_returns_empty_slice() {
    struct SliceWrapper;

    impl SliceWrapper {
        pub const fn from_slice(_: &[()]) -> &'static [()] {
            &[]
        }

        pub const fn new<'a>() -> &'a [()] {
            Self::from_slice(&[])
        }
    }

    let result: &'static [()] = SliceWrapper::new();
    assert_eq!(result.len(), 0);
}

