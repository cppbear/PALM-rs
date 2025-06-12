// Answer 0

#[test]
fn test_fmt() {
    struct MaxSizeReached;

    let instance = MaxSizeReached;

    let mut output = std::fmt::Formatter::new();
    let result = instance.fmt(&mut output);

    assert!(result.is_ok());
}

