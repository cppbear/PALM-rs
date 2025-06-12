// Answer 0

#[test]
fn test_new_empty_slice() {
    struct SliceWrapper;

    impl SliceWrapper {
        pub const fn from_slice(_: &[()]) -> &'static Self {
            &Self
        }
    }

    let result: &'static SliceWrapper = SliceWrapper::from_slice(&[]);
    assert_eq!(result as *const _, SliceWrapper::from_slice(&[]) as *const _);
}

#[test]
fn test_new_return_type() {
    struct SliceWrapper;

    impl SliceWrapper {
        pub const fn from_slice(_: &[()]) -> &'static Self {
            &Self
        }
    }

    let result: &'static SliceWrapper = SliceWrapper::from_slice(&[]);
    assert!(std::mem::size_of::<&'static SliceWrapper>() > 0);
}

