// Answer 0

#[test]
fn test_visit_content_seq_ref_with_valid_content() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u32>;

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                if let Content::U32(num) = value {
                    vec.push(num);
                }
            }
            Ok(vec)
        }
    }

    let content = vec![
        Content::U32(1),
        Content::U32(2),
        Content::U32(3),
    ];

    let result: Result<Vec<u32>, _> = visit_content_seq_ref(&content, TestVisitor);
    assert_eq!(result, Ok(vec![1, 2, 3]));
}

#[test]
fn test_visit_content_seq_ref_with_empty_content() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u32>;

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                if let Content::U32(num) = value {
                    vec.push(num);
                }
            }
            Ok(vec)
        }
    }

    let content: Vec<Content> = vec![];

    let result: Result<Vec<u32>, _> = visit_content_seq_ref(&content, TestVisitor);
    assert_eq!(result, Ok(vec![]));
}

#[test]
#[should_panic]
fn test_visit_content_seq_ref_with_panic_on_seq_end() {
    struct PanicVisitor;

    impl<'de> Visitor<'de> for PanicVisitor {
        type Value = ();

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            // Intentionally panicking to check the behavior
            panic!("panic in visit_seq");
        }
    }

    let content = vec![
        Content::U32(1),
        Content::U32(2),
    ];

    let _ = visit_content_seq_ref(&content, PanicVisitor);
}

#[test]
fn test_visit_content_seq_ref_with_mixed_content() {
    struct MixedVisitor;

    impl<'de> Visitor<'de> for MixedVisitor {
        type Value = Vec<String>;

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                match value {
                    Content::String(s) => vec.push(s),
                    _ => (),
                }
            }
            Ok(vec)
        }
    }

    let content = vec![
        Content::String("foo".to_string()),
        Content::String("bar".to_string()),
        Content::U32(3),
    ];

    let result: Result<Vec<String>, _> = visit_content_seq_ref(&content, MixedVisitor);
    assert_eq!(result, Ok(vec!["foo".to_string(), "bar".to_string()]));
}

