// Answer 0

#[test]
fn test_stream_deserializer_from_str() {
    use serde_json::Deserializer;
    use serde_json::stream::StreamDeserializer;
    use std::io::Cursor;

    let json_str = r#"[{"name": "Alice"}, {"name": "Bob"}]"#;
    let cursor = Cursor::new(json_str.as_bytes());
    let stream_deserializer: StreamDeserializer<_, serde_json::Value> = StreamDeserializer::new(cursor);

    let results: Vec<_> = stream_deserializer.collect();
    let expected = vec![
        serde_json::from_str(r#"{"name": "Alice"}"#).unwrap(),
        serde_json::from_str(r#"{"name": "Bob"}"#).unwrap(),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_stream_deserializer_from_slice() {
    use serde_json::Deserializer;
    use serde_json::stream::StreamDeserializer;

    let json_slice: &[u8] = br#"[{"age": 30}, {"age": 25}]"#;
    let stream_deserializer: StreamDeserializer<_, serde_json::Value> = StreamDeserializer::new(json_slice);

    let results: Vec<_> = stream_deserializer.collect();
    let expected = vec![
        serde_json::from_str(r#"{"age": 30}"#).unwrap(),
        serde_json::from_str(r#"{"age": 25}"#).unwrap(),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_stream_deserializer_failure() {
    use serde_json::Deserializer;
    use serde_json::stream::StreamDeserializer;
    use std::io::Cursor;

    let invalid_json_str = r#"{"name": "Alice" "age": 30}"#; // Missing comma
    let cursor = Cursor::new(invalid_json_str.as_bytes());
    let mut stream_deserializer: StreamDeserializer<_, serde_json::Value> = StreamDeserializer::new(cursor);

    let results: Vec<_> = stream_deserializer.collect();
    assert!(results.is_empty());
}

