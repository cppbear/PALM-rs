// Answer 0

#[test]
fn test_new_returns_empty_slice() {
    struct EmptySlice;
    impl EmptySlice {
        pub const fn from_slice(slice: &'_ [()]) -> &'_ Self {
            &EmptySlice
        }
    }

    let result: &EmptySlice = EmptySlice::new();
    assert!(result as *const _ != std::ptr::null());
}

