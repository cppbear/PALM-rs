// Answer 0

#[test]
fn test_tagged_content_visitor_new() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Test Visitor")
        }
    }

    let visitor = TaggedContentVisitor::<TestVisitor>::new("test_tag", "expecting_test");
    assert_eq!(visitor.tag_name, "test_tag");
    assert_eq!(visitor.expecting, "expecting_test");
}

#[test]
fn test_tagged_content_visitor_with_different_tags() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Test Visitor")
        }
    }

    let visitor1 = TaggedContentVisitor::<TestVisitor>::new("tag1", "expect1");
    let visitor2 = TaggedContentVisitor::<TestVisitor>::new("tag2", "expect2");

    assert_ne!(visitor1.tag_name, visitor2.tag_name);
    assert_ne!(visitor1.expecting, visitor2.expecting);
}

#[test]
fn test_tagged_content_visitor_empty_tags() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Test Visitor")
        }
    }

    let visitor = TaggedContentVisitor::<TestVisitor>::new("", "");
    assert_eq!(visitor.tag_name, "");
    assert_eq!(visitor.expecting, "");
}

#[test]
fn test_tagged_content_visitor_static_lifetime() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Test Visitor")
        }
    }

    let name: &'static str = "static_tag";
    let expecting: &'static str = "static_expectation";
    let visitor = TaggedContentVisitor::<TestVisitor>::new(name, expecting);
    assert_eq!(visitor.tag_name, name);
    assert_eq!(visitor.expecting, expecting);
}

