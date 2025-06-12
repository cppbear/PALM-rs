// Answer 0

#[derive(Default)]
struct TestSerializer {
    key: Option<String>,
}

impl TestSerializer {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize)]
struct TestKey<'a> {
    value: &'a str,
}

#[test]
fn test_serialize_key() {
    let mut serializer = TestSerializer::new();
    let key = TestKey { value: "test_key" };
    let result = serializer.serialize_key(&key);
    
    assert!(result.is_ok());
    assert_eq!(serializer.key.unwrap(), "test_key");
}

#[test]
#[should_panic]
fn test_serialize_key_invalid() {
    #[derive(Serialize)]
    struct InvalidKey;

    let mut serializer = TestSerializer::new();
    let key = InvalidKey;
    serializer.serialize_key(&key).unwrap();
}

