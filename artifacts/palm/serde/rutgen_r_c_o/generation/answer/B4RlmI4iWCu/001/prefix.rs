// Answer 0

#[test]
fn test_visit_bool_true() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let result = visitor.visit_bool(true);
}

#[test]
fn test_visit_bool_false() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let result = visitor.visit_bool(false);
}

