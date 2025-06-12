// Answer 0

#[test]
fn test_visit_str_with_matching_tag_name() {
    let visitor = TagOrContentVisitor {
        name: "tag_name",
        value: PhantomData,
    };
    let _ = visitor.visit_str("tag_name");
}

#[test]
fn test_visit_str_with_non_matching_string() {
    let visitor = TagOrContentVisitor {
        name: "tag_name",
        value: PhantomData,
    };
    let _ = visitor.visit_str("different_name");
}

