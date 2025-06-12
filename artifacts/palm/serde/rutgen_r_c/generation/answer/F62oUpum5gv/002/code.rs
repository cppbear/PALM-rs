// Answer 0

#[test]
fn test_visit_content_seq_err_on_seq_visitor_end() {
    use std::marker::PhantomData;

    struct TestVisitor {
        _marker: PhantomData<()>
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            Err(de::Error::custom("Visit sequence error"))
        }
        
        // Implementing the required methods with no-op for simplicity.
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence")
        }
    }

    let content = vec![Content::Bool(true), Content::U8(42)];
    let visitor = TestVisitor { _marker: PhantomData };

    let result: Result<(), _> = visit_content_seq(content, visitor);
    assert!(result.is_err());
}

#[test]
fn test_visit_content_seq_err_on_seq_visitor_visit() {
    use std::marker::PhantomData;

    struct TestVisitor {
        _marker: PhantomData<()>
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            // Simulating an error instead of successful visit
            Err(<V::Error as de::Error>::custom("Simulated error"))
        }
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence")
        }
    }

    let content = vec![Content::I32(10)];
    let visitor = TestVisitor { _marker: PhantomData };

    let result: Result<(), _> = visit_content_seq(content, visitor);
    assert!(result.is_err());
}

