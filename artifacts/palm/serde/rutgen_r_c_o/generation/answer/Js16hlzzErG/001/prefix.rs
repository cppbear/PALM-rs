// Answer 0

#[test]
fn test_visit_bool_true() {
    let visitor = BoolVisitor;
    let result = visitor.visit_bool(true);
}

#[test]
fn test_visit_bool_false() {
    let visitor = BoolVisitor;
    let result = visitor.visit_bool(false);
}

