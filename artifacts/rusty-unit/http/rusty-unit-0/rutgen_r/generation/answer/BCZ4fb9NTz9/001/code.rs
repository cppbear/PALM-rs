// Answer 0

#[test]
fn test_fmt_max_size_reached() {
    use std::fmt;

    struct MaxSizeReached;

    impl fmt::Display for MaxSizeReached {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("max size reached")
        }
    }

    let instance = MaxSizeReached;
    let mut output = String::new();
    let result = write!(&mut output, "{}", instance);

    assert!(result.is_ok());
    assert_eq!(output, "max size reached");
}

