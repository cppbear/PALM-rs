// Answer 0

#[test]
fn test_fmt_debug_struct() {
    let max_size_reached = MaxSizeReached { _priv: () };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", max_size_reached);
    assert!(result.is_ok());
    assert_eq!(output, "MaxSizeReached");
}

