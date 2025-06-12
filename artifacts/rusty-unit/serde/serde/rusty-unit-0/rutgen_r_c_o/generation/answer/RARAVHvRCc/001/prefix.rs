// Answer 0

#[test]
fn test_expecting_empty_array() {
    let mut formatter = std::fmt::Formatter::new(&mut String::new());
    let array_visitor = ArrayVisitor::<[(); 0]>::default();
    array_visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_empty_array_with_formatter() {
    let mut formatter = String::new();
    let array_visitor = ArrayVisitor::<[(); 0]>::default();
    array_visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_empty_array_print() {
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        let array_visitor = ArrayVisitor::<[(); 0]>::default();
        array_visitor.expecting(&mut formatter);
    }
    assert_eq!(output, "an empty array");
}

