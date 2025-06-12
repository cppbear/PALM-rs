// Answer 0

#[derive(Debug)]
struct TestSerializer;

impl TestSerializer {
    fn serialize_i8(self, _: i8) -> Result<(), String> {
        Err("Bad type: Unsupported Integer".to_string())
    }

    fn bad_type(_: Unsupported) -> String {
        "Bad type: Unsupported Integer".to_string()
    }
}

#[derive(Debug)]
struct Unsupported;

#[test]
fn test_serialize_i8_bad_type() {
    let serializer = TestSerializer;
    let result = serializer.serialize_i8(42);
    assert_eq!(result, Err("Bad type: Unsupported Integer".to_string()));
}

