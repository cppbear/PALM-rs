// Answer 0

#[test]
fn test_visit_f64_negative() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_f64(-1.7976931348623157e308);
}

#[test]
fn test_visit_f64_zero() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_f64(0.0);
}

#[test]
fn test_visit_f64_positive() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_f64(1.7976931348623157e308);
}

