// Answer 0

#[test]
fn test_fmt_output_slice_too_small() {
    use std::fmt;

    struct Error;

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self => write!(f, "Output slice too small"),
            }
        }
    }

    let error = Error;
    let result = format!("{}", error);
    assert_eq!(result, "Output slice too small");
}

