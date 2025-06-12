// Answer 0

#[test]
fn test_visit_content_map_success() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, E>
        where
            M: MapAccess<'de>,
        {
            Ok("Success".to_string())
        }

        // Implement other required methods from Visitor trait with no-op or default behavior
    }

    let content = vec![
        (Content::String("key1".to_string()), Content::String("value1".to_string())),
        (Content::String("key2".to_string()), Content::String("value2".to_string())),
    ];

    let result: Result<String, _> = visit_content_map(content, TestVisitor);
    assert_eq!(result, Ok("Success".to_string()));
}

#[test]
fn test_visit_content_map_end_error() {
    struct ErrorVisitor;

    impl<'de> Visitor<'de> for ErrorVisitor {
        type Value = ();

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, E>
        where
            M: MapAccess<'de>,
        {
            // Simulate an error when ending the visitor
            Err(de::Error::custom("visit_map error"))
        }

        // Implement other required methods from Visitor trait with no-op or default behavior
    }

    let content = vec![
        (Content::String("key1".to_string()), Content::String("value1".to_string())),
    ];

    let result: Result<(), _> = visit_content_map(content, ErrorVisitor);
    assert!(result.is_err());
}

