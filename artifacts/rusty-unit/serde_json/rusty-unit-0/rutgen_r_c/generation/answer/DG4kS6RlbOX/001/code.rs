// Answer 0

#[test]
fn test_to_writer_string() {
    use std::io::Cursor;
    use serde_json::json;

    let json_value = json!({"key": "value"});
    let mut output = Cursor::new(Vec::new());

    let result = to_writer(&mut output, &json_value);
    assert!(result.is_ok());

    let expected_output = b"{\"key\":\"value\"}";
    assert_eq!(output.get_ref(), expected_output);
}

#[test]
fn test_to_writer_empty_object() {
    use std::io::Cursor;
    use serde_json::json;

    let json_value = json!({});
    let mut output = Cursor::new(Vec::new());

    let result = to_writer(&mut output, &json_value);
    assert!(result.is_ok());

    let expected_output = b"{}";
    assert_eq!(output.get_ref(), expected_output);
}

#[test]
fn test_to_writer_non_string_key() {
    use std::io::Cursor;
    use serde::ser::Serialize;
    use serde::SerializeStruct;

    struct NonStringKey {
        key: i32,
    }

    impl Serialize for NonStringKey {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut state = serializer.serialize_struct("NonStringKey", 1)?;
            state.serialize_field(&self.key.to_string(), "value")?;
            state.end()
        }
    }

    let json_value = NonStringKey { key: 42 };
    let mut output = Cursor::new(Vec::new());

    let result = to_writer(&mut output, &json_value);
    assert!(result.is_ok());

    let expected_output = b"{\"42\":\"value\"}";
    assert_eq!(output.get_ref(), expected_output);
}

#[test]
#[should_panic]
fn test_to_writer_invalid_utf8() {
    use std::io::Cursor;

    struct NonUtf8String(Vec<u8>);

    impl Serialize for NonUtf8String {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_bytes(&self.0)
        }
    }

    let non_utf8_value = NonUtf8String(vec![0, 159, 146, 150]); // Invalid UTF-8
    let mut output = Cursor::new(Vec::new());

    let _ = to_writer(&mut output, &non_utf8_value); // This should panic
}

