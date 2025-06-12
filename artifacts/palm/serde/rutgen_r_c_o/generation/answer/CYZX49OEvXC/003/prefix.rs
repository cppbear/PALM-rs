// Answer 0

#[test]
fn test_visit_bytes_not_tag_or_content() {
    let visitor = TagContentOtherFieldVisitor {
        tag: "example_tag",
        content: "example_content",
    };
    let input = b"non_matching_bytes";
    visitor.visit_bytes(input);
}

#[test]
fn test_visit_bytes_not_tag_or_content_empty() {
    let visitor = TagContentOtherFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let input = b"";
    visitor.visit_bytes(input);
}

#[test]
fn test_visit_bytes_not_tag_or_content_special_chars() {
    let visitor = TagContentOtherFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let input = b"!@#$%^&*()_+";
    visitor.visit_bytes(input);
}

#[test]
fn test_visit_bytes_not_tag_or_content_numeric() {
    let visitor = TagContentOtherFieldVisitor {
        tag: "1234",
        content: "5678",
    };
    let input = b"91011";
    visitor.visit_bytes(input);
}

#[test]
fn test_visit_bytes_not_tag_or_content_unicode() {
    let visitor = TagContentOtherFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let input = "こんにちは".as_bytes();
    visitor.visit_bytes(input);
}

