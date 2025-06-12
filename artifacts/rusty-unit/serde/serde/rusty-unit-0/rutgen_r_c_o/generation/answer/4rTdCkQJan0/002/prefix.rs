// Answer 0

#[test]
fn test_visit_str_content_variants() {
    let visitor = TagOrContentFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let _ = visitor.visit_str("content");
    let _ = visitor.visit_str("Content");
    let _ = visitor.visit_str("coNtEnT");
    let _ = visitor.visit_str("coNtent");
}

