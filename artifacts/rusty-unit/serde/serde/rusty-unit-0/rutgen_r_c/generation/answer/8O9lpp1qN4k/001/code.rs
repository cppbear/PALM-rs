// Answer 0

#[test]
fn test_unexpected_map() {
    struct TestContent<'de> {
        content: Content<'de>,
    }

    let test_content = TestContent {
        content: Content::Map(vec![(Content::String("key1".to_string()), Content::U32(42))]),
    };

    let result = test_content.content.unexpected();
    
    match result {
        Unexpected::Map => (),
        _ => panic!("Expected Unexpected::Map, got {:?}", result),
    }
}

#[test]
fn test_unexpected_map_empty() {
    struct TestContent<'de> {
        content: Content<'de>,
    }

    let test_content = TestContent {
        content: Content::Map(vec![]),
    };

    let result = test_content.content.unexpected();
    
    match result {
        Unexpected::Map => (),
        _ => panic!("Expected Unexpected::Map, got {:?}", result),
    }
}

#[test]
fn test_unexpected_map_with_nested() {
    struct TestContent<'de> {
        content: Content<'de>,
    }

    let test_content = TestContent {
        content: Content::Map(vec![
            (Content::String("key1".to_string()), Content::Seq(vec![Content::U8(1), Content::U8(2)])),
            (Content::String("key2".to_string()), Content::Seq(vec![Content::U8(3)])),
        ]),
    };

    let result = test_content.content.unexpected();
    
    match result {
        Unexpected::Map => (),
        _ => panic!("Expected Unexpected::Map, got {:?}", result),
    }
}

