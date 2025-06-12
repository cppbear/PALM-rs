// Answer 0

#[test]
fn test_visit_bool_true() {
    let visitor = IgnoredAny;
    let result = visitor.visit_bool(true);
}

#[test]
fn test_visit_bool_false() {
    let visitor = IgnoredAny;
    let result = visitor.visit_bool(false);
}

