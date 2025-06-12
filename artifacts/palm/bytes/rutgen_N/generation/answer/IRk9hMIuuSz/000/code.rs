// Answer 0

#[test]
fn test_fmt_uninit_slice() {
    struct UninitSlice;

    impl std::fmt::Debug for UninitSlice {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fmt.debug_struct("UninitSlice[...]").finish()
        }
    }

    let uninit_slice = UninitSlice;
    let result = format!("{:?}", uninit_slice);
    assert_eq!(result, "UninitSlice[...]");
}

