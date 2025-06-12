// Answer 0

#[test]
fn test_tag_or_content_visitor_creation() {
    struct TestVisitor<'de> {
        name: &'static str,
        value: std::marker::PhantomData<TagOrContent<'de>>,
    }

    impl<'de> TagOrContentVisitor<'de> {
        fn new(name: &'static str) -> Self {
            TagOrContentVisitor {
                name,
                value: std::marker::PhantomData,
            }
        }
    }

    let visitor = TestVisitor::new("test_name");

    assert_eq!(visitor.name, "test_name");
}

