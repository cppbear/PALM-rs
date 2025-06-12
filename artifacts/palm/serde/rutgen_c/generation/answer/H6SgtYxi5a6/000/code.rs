// Answer 0

#[test]
fn test_fmt_one_element() {
    let expected = ExpectedInMap(1);
    let mut output = String::new();
    let result = expected.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "1 element in map");
}

#[test]
fn test_fmt_multiple_elements() {
    let expected = ExpectedInMap(5);
    let mut output = String::new();
    let result = expected.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "5 elements in map");
}

