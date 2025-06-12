// Answer 0

#[test]
fn test_visit_u64_field_index_0() {
    let visitor = TagOrContentFieldVisitor { tag: "tag", content: "content" };
    let _ = visitor.visit_u64(0);
}

#[test]
fn test_visit_u64_field_index_1() {
    let visitor = TagOrContentFieldVisitor { tag: "tag", content: "content" };
    let _ = visitor.visit_u64(1);
}

#[should_panic]
fn test_visit_u64_field_index_2() {
    let visitor = TagOrContentFieldVisitor { tag: "tag", content: "content" };
    let _ = visitor.visit_u64(2);
}

