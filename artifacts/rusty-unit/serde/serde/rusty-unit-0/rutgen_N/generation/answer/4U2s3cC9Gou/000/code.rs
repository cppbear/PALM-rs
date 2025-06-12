// Answer 0

#[derive(Debug)]
struct TestSerializer;

impl TestSerializer {
    fn bad_type(&self, _: Unsupported) -> std::result::Result<(), String> {
        Err("Unsupported type".to_string())
    }
}

#[derive(Debug)]
enum Unsupported {
    Integer,
}

impl TestSerializer {
    type Ok = ();
    type Error = String;

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }
}

#[test]
fn test_serialize_u64_returns_error() {
    let serializer = TestSerializer;
    let result = serializer.serialize_u64(42);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Unsupported type");
}

