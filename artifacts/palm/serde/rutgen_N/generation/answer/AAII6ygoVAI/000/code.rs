// Answer 0

#[derive(Debug)]
struct TestStruct {
    value: String,
}

impl TestStruct {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl serde::Serialize for TestStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.value)
    }
}

#[test]
fn test_serialize_empty_string() {
    let test_struct = TestStruct::new("");
    let serialized = serde_json::to_string(&test_struct).unwrap();
    assert_eq!(serialized, "\"\"");
}

#[test]
fn test_serialize_non_empty_string() {
    let test_struct = TestStruct::new("Hello, world!");
    let serialized = serde_json::to_string(&test_struct).unwrap();
    assert_eq!(serialized, "\"Hello, world!\"");
}

#[test]
fn test_serialize_special_characters() {
    let test_struct = TestStruct::new("Line1\nLine2\tTab");
    let serialized = serde_json::to_string(&test_struct).unwrap();
    assert_eq!(serialized, "\"Line1\\nLine2\\tTab\"");
}

