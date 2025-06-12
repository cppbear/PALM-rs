// Answer 0

#[test]
fn test_visit_content_seq_err() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_seq<V>(self, _seq: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            Err(de::Error::custom("Visitor error"))
        }
    }

    let content = vec![Content::Bool(true), Content::U32(42)];

    let result: Result<(), _> = visit_content_seq(content, TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_visit_content_seq_empty() {
    struct EmptyVisitor;

    impl<'de> Visitor<'de> for EmptyVisitor {
        type Value = ();

        fn visit_seq<V>(self, _seq: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let content: Vec<Content> = Vec::new();

    let result: Result<(), _> = visit_content_seq(content, EmptyVisitor);
    assert!(result.is_ok());
}

