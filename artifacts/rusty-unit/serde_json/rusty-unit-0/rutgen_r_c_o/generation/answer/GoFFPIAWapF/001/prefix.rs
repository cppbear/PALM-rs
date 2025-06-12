// Answer 0

#[test]
fn test_from_reader_empty_json() {
    let json_data = b"{}";
    let reader = std::io::Cursor::new(json_data);
    let _: serde_json::Value = serde_json::from_reader(reader).unwrap();
}

#[test]
fn test_from_reader_basic_struct() {
    let json_data = br#"{"fingerprint":"abc123","location":"earth"}"#;
    let reader = std::io::Cursor::new(json_data);
    
    #[derive(serde::Deserialize)]
    struct User {
        fingerprint: String,
        location: String,
    }
    
    let _: User = serde_json::from_reader(reader).unwrap();
}

#[test]
fn test_from_reader_large_json() {
    let json_data = br#"{"field1":"a","field2":"b","field3":"c","field4":"d","field5":"e"}"#;
    let reader = std::io::Cursor::new(json_data);
    
    #[derive(serde::Deserialize)]
    struct LargeStruct {
        field1: String,
        field2: String,
        field3: String,
        field4: String,
        field5: String,
    }
    
    let _: LargeStruct = serde_json::from_reader(reader).unwrap();
}

#[test]
fn test_from_reader_struct_with_empty_strings() {
    let json_data = br#"{"fingerprint":"","location":""}"#;
    let reader = std::io::Cursor::new(json_data);
    
    #[derive(serde::Deserialize)]
    struct User {
        fingerprint: String,
        location: String,
    }
    
    let _: User = serde_json::from_reader(reader).unwrap();
}

#[test]
fn test_from_reader_struct_with_max_length_string() {
    let long_string = "a".repeat(256); // maximum length for string
    let json_data = format!(r#"{{"fingerprint":"{}","location":"earth"}}"#, long_string);
    let reader = std::io::Cursor::new(json_data.as_bytes());

    #[derive(serde::Deserialize)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let _: User = serde_json::from_reader(reader).unwrap();
}

#[test]
fn test_from_reader_incorrect_json_format() {
    let json_data = b"{fingerprint:\"abc123\",location:\"earth\"}";
    let reader = std::io::Cursor::new(json_data);

    #[derive(serde::Deserialize)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let result: Result<User, _> = serde_json::from_reader(reader);
    assert!(result.is_err());
}

#[test]
fn test_from_reader_nested_struct() {
    let json_data = br#"{"user":{"fingerprint":"abc123","location":"earth"}}"#;
    let reader = std::io::Cursor::new(json_data);

    #[derive(serde::Deserialize)]
    struct User {
        fingerprint: String,
        location: String,
    }

    #[derive(serde::Deserialize)]
    struct Response {
        user: User,
    }
    
    let _: Response = serde_json::from_reader(reader).unwrap();
}

#[test]
fn test_from_reader_large_number_of_fields() {
    let json_data = br#"{"field1":"value1","field2":"value2","field3":"value3","field4":"value4","field5":"value5","field6":"value6","field7":"value7","field8":"value8","field9":"value9","field10":"value10"}"#;
    let reader = std::io::Cursor::new(json_data);

    #[derive(serde::Deserialize)]
    struct ManyFields {
        field1: String,
        field2: String,
        field3: String,
        field4: String,
        field5: String,
        field6: String,
        field7: String,
        field8: String,
        field9: String,
        field10: String,
    }

    let _: ManyFields = serde_json::from_reader(reader).unwrap();
}

#[test]
fn test_from_reader_too_large_data() {
    let json_data = r#"{"big_field":[1,2,3,4,5,6,7,8,9,10]},"#; // intentionally malformed
    let reader = std::io::Cursor::new(json_data);

    #[derive(serde::Deserialize)]
    struct Data {
        big_field: Vec<u8>,
    }

    let result: Result<Data, _> = serde_json::from_reader(reader);
    assert!(result.is_err());
}

