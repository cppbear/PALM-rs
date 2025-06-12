// Answer 0

#[test]
fn test_visit_i64_min() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let _ = visitor.visit_i64(-9223372036854775808);
}

#[test]
fn test_visit_i64_negative() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let _ = visitor.visit_i64(-1);
}

#[test]
fn test_visit_i64_zero() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let _ = visitor.visit_i64(0);
}

#[test]
fn test_visit_i64_positive() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let _ = visitor.visit_i64(1);
}

#[test]
fn test_visit_i64_max() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let _ = visitor.visit_i64(9223372036854775807);
}

