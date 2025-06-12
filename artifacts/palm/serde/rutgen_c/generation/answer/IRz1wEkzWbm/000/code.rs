// Answer 0

#[test]
fn test_visit_u64_tag() {
    let visitor = TagOrContentFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let result = visitor.visit_u64(0);
    assert_eq!(result, Ok(TagOrContentField::Tag));
}

#[test]
fn test_visit_u64_content() {
    let visitor = TagOrContentFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let result = visitor.visit_u64(1);
    assert_eq!(result, Ok(TagOrContentField::Content));
}

#[test]
fn test_visit_u64_invalid() {
    let visitor = TagOrContentFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let result = visitor.visit_u64(2);
    assert!(result.is_err());
}

