// Answer 0

#[test]
fn test_visit_str_empty() {
    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, _> = visitor.visit_str("");
    assert_eq!(result, Ok(Content::String("".into())));
}

#[test]
fn test_visit_str_basic() {
    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, _> = visitor.visit_str("Hello");
    assert_eq!(result, Ok(Content::String("Hello".into())));
}

#[test]
fn test_visit_str_unicode() {
    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, _> = visitor.visit_str("こんにちは");
    assert_eq!(result, Ok(Content::String("こんにちは".into())));
}

#[test]
fn test_visit_str_special_characters() {
    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, _> = visitor.visit_str("!@#$%^&*()");
    assert_eq!(result, Ok(Content::String("!@#$%^&*()".into())));
}

#[test]
fn test_visit_str_long_string() {
    let visitor = ContentVisitor { value: PhantomData };
    let long_string = "a".repeat(1000);
    let result: Result<Content, _> = visitor.visit_str(&long_string);
    assert_eq!(result, Ok(Content::String(long_string)));
}

