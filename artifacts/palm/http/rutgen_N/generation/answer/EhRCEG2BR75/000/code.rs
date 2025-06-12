// Answer 0

#[test]
fn test_invalid_header_name_debug() {
    use std::fmt;

    struct InvalidHeaderName;

    impl fmt::Debug for InvalidHeaderName {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("InvalidHeaderName").finish()
        }
    }

    let header_name = InvalidHeaderName;
    let result = format!("{:?}", header_name);
    assert_eq!(result, "InvalidHeaderName");
}

