// Answer 0

#[test]
fn test_expecting_empty_array() {
    let mut buf = String::new();
    let formatter = &mut buf;
    let visitor = CStringVisitor;
    visitor.expecting(formatter);
}

#[test]
fn test_expecting_single_byte() {
    let mut buf = String::new();
    let formatter = &mut buf;
    let visitor = CStringVisitor;
    visitor.expecting(formatter);
}

#[test]
fn test_expecting_multiple_bytes() {
    let mut buf = String::new();
    let formatter = &mut buf;
    let visitor = CStringVisitor;
    visitor.expecting(formatter);
}

#[test]
fn test_expecting_max_byte_value() {
    let mut buf = String::new();
    let formatter = &mut buf;
    let visitor = CStringVisitor;
    visitor.expecting(formatter);
}

#[test]
fn test_expecting_range_of_bytes() {
    let mut buf = String::new();
    let formatter = &mut buf;
    let visitor = CStringVisitor;
    visitor.expecting(formatter);
}

#[test]
#[should_panic]
fn test_expecting_invalid_byte() {
    let mut buf = String::new();
    let formatter = &mut format!("invalid byte value");
    let visitor = CStringVisitor;
    visitor.expecting(formatter);
}

