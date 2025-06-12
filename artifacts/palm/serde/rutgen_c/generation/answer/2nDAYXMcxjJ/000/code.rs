// Answer 0

#[test]
fn test_expectation_str_visitor() {
    let visitor = StrVisitor;
    let mut output = String::new();
    let result = visitor.expecting(&mut output).unwrap();
    assert_eq!(result, ());
    assert_eq!(output, "a borrowed string");
}

