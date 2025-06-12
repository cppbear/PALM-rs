// Answer 0

#[test]
fn test_from_str_visitor_new() {
    let expecting = "some type";
    let visitor = FromStrVisitor::<u32>::new(expecting);
    assert_eq!(visitor.expecting, expecting);
}

#[test]
fn test_from_str_visitor_new_empty() {
    let expecting = "";
    let visitor = FromStrVisitor::<String>::new(expecting);
    assert_eq!(visitor.expecting, expecting);
}

#[test]
fn test_from_str_visitor_new_long_string() {
    let expecting = "a very long string that describes what we're expecting";
    let visitor = FromStrVisitor::<f64>::new(expecting);
    assert_eq!(visitor.expecting, expecting);
}

