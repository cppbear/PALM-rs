// Answer 0

#[test]
fn test_visit_content_seq_with_error_on_end() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let content = vec![
        Content::String("test".to_string()),
        Content::I32(5),
        Content::None,
    ];

    let visitor = TestVisitor;

    let result = visit_content_seq(content, visitor);
}

#[test]
fn test_visit_content_seq_with_partial_content() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let content = vec![
        Content::String("test".to_string()),
        Content::I32(5),
    ];

    let visitor = TestVisitor;

    let result = visit_content_seq(content, visitor);
}

