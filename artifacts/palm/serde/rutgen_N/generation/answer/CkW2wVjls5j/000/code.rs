// Answer 0

#[test]
fn test_new() {
    struct TaggedContentVisitor<'a> {
        tag_name: &'a str,
        expecting: &'a str,
        value: std::marker::PhantomData<()>,
    }

    let visitor = TaggedContentVisitor::new("my_tag", "my_expecting");
    assert_eq!(visitor.tag_name, "my_tag");
    assert_eq!(visitor.expecting, "my_expecting");
}

