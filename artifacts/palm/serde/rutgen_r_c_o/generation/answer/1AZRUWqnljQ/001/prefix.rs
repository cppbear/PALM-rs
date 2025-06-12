// Answer 0

#[test]
fn test_visit_bytes_tag_match() {
    let visitor = TagOrContentFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let _ = visitor.visit_bytes(b"tag");
}

#[test]
fn test_visit_bytes_content_match() {
    let visitor = TagOrContentFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let _ = visitor.visit_bytes(b"content");
}

#[test]
fn test_visit_bytes_off_by_one_tag() {
    let visitor = TagOrContentFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let _ = visitor.visit_bytes(b"tagx");
}

#[test]
fn test_visit_bytes_off_by_one_content() {
    let visitor = TagOrContentFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let _ = visitor.visit_bytes(b"contentx");
}

#[test]
fn test_visit_bytes_not_match() {
    let visitor = TagOrContentFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let _ = visitor.visit_bytes(b"ggg");
}

