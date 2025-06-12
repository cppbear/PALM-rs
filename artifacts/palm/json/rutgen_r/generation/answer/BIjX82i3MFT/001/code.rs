// Answer 0

#[test]
fn test_into_iter_with_valid_json() {
    use serde_json::de::{Deserializer, StreamDeserializer};
    use serde::Deserialize;
    
    #[derive(Deserialize)]
    struct TestStruct {
        field: String,
    }
    
    let json_data = r#"[{"field": "value1"}, {"field": "value2"}]"#;
    let deserializer = Deserializer::from_str(json_data);
    
    let iter: StreamDeserializer<_, TestStruct> = deserializer.into_iter();
    let results: Vec<TestStruct> = iter.collect::<Result<_, _>>().unwrap();
    
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].field, "value1");
    assert_eq!(results[1].field, "value2");
}

#[test]
#[should_panic]
fn test_into_iter_with_invalid_json() {
    use serde_json::de::{Deserializer, StreamDeserializer};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct TestStruct {
        field: String,
    }
    
    let json_data = r#"invalid json"#;
    let deserializer = Deserializer::from_str(json_data);
    
    let iter: StreamDeserializer<_, TestStruct> = deserializer.into_iter();
    let _: Vec<TestStruct> = iter.collect::<Result<_, _>>().unwrap();
}

