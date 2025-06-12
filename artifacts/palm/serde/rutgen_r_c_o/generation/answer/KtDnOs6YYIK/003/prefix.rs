// Answer 0

#[test]
fn test_visit_u64_zero() {
    let visitor = TagContentOtherFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let _ = visitor.visit_u64(0);
}

#[test]
fn test_visit_u64_one() {
    let visitor = TagContentOtherFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let _ = visitor.visit_u64(1);
}

#[test]
fn test_visit_u64_other() {
    let visitor = TagContentOtherFieldVisitor {
        tag: "tag",
        content: "content",
    };
    let _ = visitor.visit_u64(2);
}

