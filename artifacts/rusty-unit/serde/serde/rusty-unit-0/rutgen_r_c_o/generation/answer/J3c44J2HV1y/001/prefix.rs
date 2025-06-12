// Answer 0

#[test]
fn test_visit_i64_min() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_i64(-9223372036854775808);
}

#[test]
fn test_visit_i64_zero() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_i64(0);
}

#[test]
fn test_visit_i64_positive() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_i64(12345);
}

#[test]
fn test_visit_i64_max() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_i64(9223372036854775807);
}

#[test]
fn test_visit_i64_negative() {
    let visitor = IgnoredAny;
    let _ = visitor.visit_i64(-12345);
}

