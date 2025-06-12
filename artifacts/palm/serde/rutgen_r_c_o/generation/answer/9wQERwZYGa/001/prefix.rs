// Answer 0

#[test]
fn test_visit_f64_positive() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let _ = visitor.visit_f64(1.0);
}

#[test]
fn test_visit_f64_negative() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let _ = visitor.visit_f64(-1.0);
}

#[test]
fn test_visit_f64_zero() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let _ = visitor.visit_f64(0.0);
}

#[test]
fn test_visit_f64_large_positive() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let _ = visitor.visit_f64(1.7976931348623157e+308);
}

#[test]
fn test_visit_f64_large_negative() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let _ = visitor.visit_f64(-1.7976931348623157e+308);
}

#[test]
fn test_visit_f64_small_positive() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let _ = visitor.visit_f64(1e-10);
}

#[test]
fn test_visit_f64_small_negative() {
    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };
    let _ = visitor.visit_f64(-1e-10);
}

