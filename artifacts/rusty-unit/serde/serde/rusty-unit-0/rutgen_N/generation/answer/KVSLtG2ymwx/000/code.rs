// Answer 0

#[derive(Debug)]
struct FromStrVisitor<'a> {
    expecting: &'a str,
    ty: std::marker::PhantomData<()>,
}

impl<'a> FromStrVisitor<'a> {
    fn new(expecting: &'static str) -> Self {
        FromStrVisitor {
            expecting,
            ty: std::marker::PhantomData,
        }
    }
}

#[test]
fn test_new_creates_fromstr_visitor() {
    let expected_str = "Expected some string";
    let visitor = FromStrVisitor::new(expected_str);
    assert_eq!(visitor.expecting, expected_str);
}

#[test]
fn test_new_with_different_strings() {
    let visitor_one = FromStrVisitor::new("First expectation");
    let visitor_two = FromStrVisitor::new("Second expectation");
    assert_ne!(visitor_one.expecting, visitor_two.expecting);
}

