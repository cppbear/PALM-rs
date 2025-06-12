// Answer 0

#[test]
fn test_visit_bytes_tag() {
    let visitor = TagOrContentFieldVisitor {
        tag: "tag_field",
        content: "content_field",
    };
    let result = visitor.visit_bytes(b"tag_field");
    assert_eq!(result, Ok(TagOrContentField::Tag));
}

#[test]
fn test_visit_bytes_content() {
    let visitor = TagOrContentFieldVisitor {
        tag: "tag_field",
        content: "content_field",
    };
    let result = visitor.visit_bytes(b"content_field");
    assert_eq!(result, Ok(TagOrContentField::Content));
}

#[test]
fn test_visit_bytes_invalid() {
    let visitor = TagOrContentFieldVisitor {
        tag: "tag_field",
        content: "content_field",
    };
    let result = visitor.visit_bytes(b"invalid_field");
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.to_string(), "invalid value: bytes \"invalid_field\", expected one of `tag_field` or `content_field`"); // Adjust the expected error message as necessary based on actual error string format
    }
}

