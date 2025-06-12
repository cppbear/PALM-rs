// Answer 0

#[test]
fn test_expected_fmt_multiple_elements() {
    let expected = ExpectedInMap(5);
    let mut output = String::new();
    let result = expected.fmt(&mut output).is_ok();
    assert!(result);
    assert_eq!(output, "5 elements in map");
}

#[test]
fn test_expected_fmt_one_element() {
    let expected = ExpectedInMap(1);
    let mut output = String::new();
    let result = expected.fmt(&mut output).is_ok();
    assert!(result);
    assert_eq!(output, "1 element in map");
}

#[test]
fn test_expected_fmt_zero_elements() {
    let expected = ExpectedInMap(0);
    let mut output = String::new();
    let result = expected.fmt(&mut output).is_ok();
    assert!(result);
    assert_eq!(output, "0 elements in map");
}

