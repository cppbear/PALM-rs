// Answer 0

#[derive(Debug)]
struct TestSerializer;

impl TestSerializer {
    fn bad_type(&self, _unsupported: Unsupported) -> Result<(), String> {
        Err("Unsupported type".to_string())
    }

    fn serialize_i8(self, _: i8) -> Result<(), String> {
        Err(self.bad_type(Unsupported::Integer))
    }
}

#[derive(Debug)]
enum Unsupported {
    Integer,
}

#[test]
fn test_serialize_i8_returns_error() {
    let serializer = TestSerializer;
    let result = serializer.serialize_i8(5);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Unsupported type".to_string());
}

