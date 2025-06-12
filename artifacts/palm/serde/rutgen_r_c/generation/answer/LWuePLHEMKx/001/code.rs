// Answer 0

#[test]
fn test_visit_string_non_empty() {
    let visitor = ContentVisitor { value: PhantomData };
    let value = String::from("Hello, world!");
    let result: Result<Content, _> = visitor.visit_string(value);
    assert_eq!(result, Ok(Content::String(String::from("Hello, world!"))));
}

#[test]
fn test_visit_string_empty() {
    let visitor = ContentVisitor { value: PhantomData };
    let value = String::from("");
    let result: Result<Content, _> = visitor.visit_string(value);
    assert_eq!(result, Ok(Content::String(String::from(""))));
}

#[test]
fn test_visit_string_special_characters() {
    let visitor = ContentVisitor { value: PhantomData };
    let value = String::from("!@#$%^&*()_+[]{}|;':\",.<>?");
    let result: Result<Content, _> = visitor.visit_string(value);
    assert_eq!(result, Ok(Content::String(String::from("!@#$%^&*()_+[]{}|;':\",.<>?"))));
}

#[test]
fn test_visit_string_unicode() {
    let visitor = ContentVisitor { value: PhantomData };
    let value = String::from("こんにちは");
    let result: Result<Content, _> = visitor.visit_string(value);
    assert_eq!(result, Ok(Content::String(String::from("こんにちは"))));
}

#[test]
fn test_visit_string_whitespace() {
    let visitor = ContentVisitor { value: PhantomData };
    let value = String::from("   ");
    let result: Result<Content, _> = visitor.visit_string(value);
    assert_eq!(result, Ok(Content::String(String::from("   "))));
}

