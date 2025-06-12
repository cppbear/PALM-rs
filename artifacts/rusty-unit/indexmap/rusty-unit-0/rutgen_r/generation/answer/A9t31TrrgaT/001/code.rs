// Answer 0

#[test]
fn test_new_empty_slice() {
    struct SliceWrapper;

    impl SliceWrapper {
        pub const fn from_slice(_: &'static [&'static str]) -> &'static Self {
            &SliceWrapper
        }

        pub const fn new<'a>() -> &'a Self {
            Self::from_slice(&[])
        }
    }

    let result = SliceWrapper::new();
    assert_eq!(result, &SliceWrapper);
}

#[test]
fn test_new_non_empty_slice() {
    struct SliceWrapper;

    impl SliceWrapper {
        pub const fn from_slice(_: &'static [&'static str]) -> &'static Self {
            &SliceWrapper
        }

        pub const fn new<'a>() -> &'a Self {
            Self::from_slice(&[])
        }
    }

    let result = SliceWrapper::new();
    assert_eq!(result, &SliceWrapper);
}

