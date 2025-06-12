// Answer 0

#[test]
fn test_unit_visitor_expectation_valid() {
    let mut buffer = String::new();
    let mut formatter = std::fmt::Formatter::new(&mut buffer);
    let visitor = UnitVisitor;
    visitor.expecting(&mut formatter);
}

#[test]
fn test_unit_visitor_expectation_output() {
    let mut buffer = String::new();
    let mut formatter = std::fmt::Formatter::new(&mut buffer);
    let visitor = UnitVisitor;
    visitor.expecting(&mut formatter);
    assert_eq!(buffer, "unit");
}

