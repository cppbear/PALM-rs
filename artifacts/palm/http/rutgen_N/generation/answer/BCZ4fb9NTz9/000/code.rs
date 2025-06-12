// Answer 0

#[test]
fn test_fmt_outputs_correct_string() {
    use std::fmt;

    struct MaxSizeReached;

    impl fmt::Display for MaxSizeReached {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("max size reached")
        }
    }

    let instance = MaxSizeReached;
    let result = format!("{}", instance);
    assert_eq!(result, "max size reached");
}

