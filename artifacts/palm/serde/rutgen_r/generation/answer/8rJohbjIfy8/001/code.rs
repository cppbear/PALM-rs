// Answer 0

#[derive(Debug)]
struct MockSerializer;

impl MockSerializer {
    fn bad_type(_: Unsupported) -> String {
        "Bad type".to_string()
    }

    fn serialize_u16(self, _: u16) -> Result<(), String> {
        Err(Self::bad_type(Unsupported::Integer))
    }
}

#[derive(Debug)]
enum Unsupported {
    Integer,
}

#[test]
fn test_serialize_u16_with_valid_input() {
    let serializer = MockSerializer;
    let result = serializer.serialize_u16(42);
    assert_eq!(result, Err("Bad type".to_string()));
}

#[test]
fn test_serialize_u16_with_boundary_input() {
    let serializer = MockSerializer;
    let result = serializer.serialize_u16(0);
    assert_eq!(result, Err("Bad type".to_string()));
}

#[test]
fn test_serialize_u16_with_max_value() {
    let serializer = MockSerializer;
    let result = serializer.serialize_u16(u16::MAX);
    assert_eq!(result, Err("Bad type".to_string()));
}

