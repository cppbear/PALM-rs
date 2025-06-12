// Answer 0

#[test]
fn test_fmt_uninit_slice() {
    use std::fmt;

    struct UninitSlice;

    impl fmt::Debug for UninitSlice {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt.debug_struct("UninitSlice[...]").finish()
        }
    }

    let uninit_slice = UninitSlice;
    let debug_output = format!("{:?}", uninit_slice);
    
    assert_eq!(debug_output, "UninitSlice[...]");
}

