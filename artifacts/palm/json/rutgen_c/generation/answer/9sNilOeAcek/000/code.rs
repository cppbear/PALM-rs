// Answer 0

#[test]
fn test_to_writer_pretty_with_valid_json() {
    use serde::ser::Serializer;
    use serde::Serialize;
    use std::io::Cursor;

    #[derive(Serialize)]
    struct TestStruct {
        name: String,
        age: u32,
    }

    let data = TestStruct {
        name: String::from("Alice"),
        age: 30,
    };

    let mut output = Cursor::new(Vec::new());
    let result = to_writer_pretty(&mut output, &data);
    assert!(result.is_ok());
    let expected_output = r#"{"name":"Alice","age":30}"#; // output may vary in whitespace
    assert_eq!(String::from_utf8(output.into_inner()).unwrap(), expected_output);
}

#[test]
#[should_panic]
fn test_to_writer_pretty_with_invalid_serialization() {
    use serde::ser::Serializer;
    use serde::Serialize;
    use std::io::Cursor;

    #[derive(Serialize)]
    struct InvalidKey {
        key: Vec<u8>,
        value: String,
    }

    let data = InvalidKey {
        key: vec![1, 2, 3], // Non-string key
        value: String::from("value"),
    };

    let mut output = Cursor::new(Vec::new());
    let _ = to_writer_pretty(&mut output, &data); // This should panic or return an error
}

