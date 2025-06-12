// Answer 0

#[test]
fn test_visit_f32_with_positive_value() {
    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, _> = visitor.visit_f32(3.14);
    assert_eq!(result, Ok(Content::F32(3.14)));
}

#[test]
fn test_visit_f32_with_negative_value() {
    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, _> = visitor.visit_f32(-2.71);
    assert_eq!(result, Ok(Content::F32(-2.71)));
}

#[test]
fn test_visit_f32_with_zero() {
    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, _> = visitor.visit_f32(0.0);
    assert_eq!(result, Ok(Content::F32(0.0)));
}

#[test]
fn test_visit_f32_with_large_value() {
    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, _> = visitor.visit_f32(1_000_000.0);
    assert_eq!(result, Ok(Content::F32(1_000_000.0)));
}

#[test]
fn test_visit_f32_with_small_value() {
    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, _> = visitor.visit_f32(1e-6);
    assert_eq!(result, Ok(Content::F32(1e-6)));
}

