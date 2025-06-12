// Answer 0

#[test]
fn test_visit_content_map_ref_successful_visit() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, E>
        where
            M: MapAccess<'de>,
        {
            Ok("map visited successfully".to_string())
        }
    }

    let content = [
        (Content::Str("key1"), Content::Str("value1")),
        (Content::Str("key2"), Content::Str("value2")),
    ];

    let result: Result<String, _> = visit_content_map_ref(&content, TestVisitor);
    assert_eq!(result, Ok("map visited successfully".to_string()));
}

#[test]
fn test_visit_content_map_ref_error_on_visit() {
    struct ErrorVisitor;

    impl<'de> Visitor<'de> for ErrorVisitor {
        type Value = String;

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, E>
        where
            M: MapAccess<'de>,
        {
            Err(de::Error::custom("visit error"))
        }
    }

    let content = [
        (Content::Str("key1"), Content::Str("value1")),
        (Content::Str("key2"), Content::Str("value2")),
    ];

    let result: Result<String, _> = visit_content_map_ref(&content, ErrorVisitor);
    assert_eq!(result, Err(de::Error::custom("visit error")));
}

#[test]
fn test_visit_content_map_ref_end_error() {
    struct EndErrorVisitor;

    impl<'de> Visitor<'de> for EndErrorVisitor {
        type Value = String;

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, E>
        where
            M: MapAccess<'de>,
        {
            Ok("map visited".to_string())
        }
    }

    let content = [
        (Content::Str("key1"), Content::Str("value1")),
        (Content::Str("key2"), Content::Str("value2")),
    ];

    let result: Result<String, _> = visit_content_map_ref(&content, EndErrorVisitor);
    assert_eq!(result, Ok("map visited".to_string()));
}

