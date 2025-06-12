// Answer 0

#[test]
fn test_tag_or_content_visitor_new() {
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

    let visitor = TagOrContentVisitor::new("test_name");
    assert_eq!(visitor.name, "test_name");
}

#[test]
fn test_tag_or_content_visitor_new_boundary() {
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

    let visitor = TagOrContentVisitor::new("");
    assert_eq!(visitor.name, "");
}

