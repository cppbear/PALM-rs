// Answer 0

#[test]
fn test_deserializer_new_valid_input() {
    use serde_json::Deserializer;
    use std::io::Cursor;

    let input_data = b"{\"key\": \"value\"}";
    let cursor = Cursor::new(input_data);

    let deserializer = Deserializer::new(cursor);

    assert_eq!(deserializer.scratch, Vec::new());
    assert_eq!(deserializer.remaining_depth, 128);
    
    #[cfg(feature = "float_roundtrip")]
    {
        assert_eq!(deserializer.single_precision, false);
    }
    
    #[cfg(feature = "unbounded_depth")]
    {
        assert_eq!(deserializer.disable_recursion_limit, false);
    }
}

#[should_panic]
#[test]
fn test_deserializer_new_invalid_reader() {
    use serde_json::Deserializer;
    use std::io::Empty;

    let empty_reader = Empty;

    let _deserializer = Deserializer::new(empty_reader);
}

