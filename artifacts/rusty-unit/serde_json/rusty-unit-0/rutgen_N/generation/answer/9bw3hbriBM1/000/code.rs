// Answer 0

#[derive(Debug)]
struct MySerializer;

impl MySerializer {
    fn serialize_tuple(self, _len: usize) -> Result<Self> {
        Err(key_must_be_a_string())
    }
}

fn key_must_be_a_string() -> String {
    "Key must be a string".to_string()
}

#[test]
fn test_serialize_tuple_err() {
    let serializer = MySerializer;
    let result = serializer.serialize_tuple(0);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Key must be a string");
}

