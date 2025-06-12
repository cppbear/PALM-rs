// Answer 0

#[test]
fn test_visit_bytes_empty() {
    let visitor = IgnoredAny;
    let result = visitor.visit_bytes(&[]);
}

#[test]
fn test_visit_bytes_single_zero() {
    let visitor = IgnoredAny;
    let result = visitor.visit_bytes(&[0]);
}

#[test]
fn test_visit_bytes_single_max() {
    let visitor = IgnoredAny;
    let result = visitor.visit_bytes(&[255]);
}

#[test]
fn test_visit_bytes_multiple() {
    let visitor = IgnoredAny;
    let result = visitor.visit_bytes(&[0, 255, 127]);
}

