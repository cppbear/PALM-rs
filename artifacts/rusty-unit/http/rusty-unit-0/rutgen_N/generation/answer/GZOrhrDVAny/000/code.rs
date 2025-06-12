// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct Extensions;

    impl fmt::Debug for Extensions {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Extensions").finish()
        }
    }

    let ext = Extensions;
    let result = format!("{:?}", ext);
    assert_eq!(result, "Extensions");
}

