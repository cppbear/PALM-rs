// Answer 0

#[test]
fn test_new_with_vec_reader() {
    use serde_json::Deserializer;
    use std::io::Cursor;

    let input: Vec<u8> = b"{}".to_vec();
    let reader = Cursor::new(input);
    let deserializer: Deserializer<Cursor<Vec<u8>>> = Deserializer::new(reader);

    assert_eq!(deserializer.remaining_depth, 128);
    assert_eq!(deserializer.scratch.len(), 0);
    #[cfg(feature = "float_roundtrip")]
    {
        assert!(!deserializer.single_precision);
    }
    #[cfg(feature = "unbounded_depth")]
    {
        assert!(!deserializer.disable_recursion_limit);
    }
}

#[test]
fn test_new_with_slice_reader() {
    use serde_json::Deserializer;

    let input: &[u8] = b"{}";
    let deserializer: Deserializer<&[u8]> = Deserializer::new(input);

    assert_eq!(deserializer.remaining_depth, 128);
    assert_eq!(deserializer.scratch.len(), 0);
    #[cfg(feature = "float_roundtrip")]
    {
        assert!(!deserializer.single_precision);
    }
    #[cfg(feature = "unbounded_depth")]
    {
        assert!(!deserializer.disable_recursion_limit);
    }
}

#[test]
fn test_new_with_empty_string() {
    use serde_json::Deserializer;
    use std::io::Cursor;

    let input: String = String::new();
    let reader = Cursor::new(input.into_bytes());
    let deserializer: Deserializer<Cursor<Vec<u8>>> = Deserializer::new(reader);

    assert_eq!(deserializer.remaining_depth, 128);
    assert_eq!(deserializer.scratch.len(), 0);
    #[cfg(feature = "float_roundtrip")]
    {
        assert!(!deserializer.single_precision);
    }
    #[cfg(feature = "unbounded_depth")]
    {
        assert!(!deserializer.disable_recursion_limit);
    }
}

