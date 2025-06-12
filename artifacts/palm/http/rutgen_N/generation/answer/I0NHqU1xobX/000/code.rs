// Answer 0

#[test]
fn test_invalid_method_fmt() {
    use std::fmt;

    struct InvalidMethod;

    impl fmt::Debug for InvalidMethod {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("InvalidMethod")
                .finish()
        }
    }

    let method = InvalidMethod;
    let result = format!("{:?}", method);
    assert_eq!(result, "InvalidMethod");
}

