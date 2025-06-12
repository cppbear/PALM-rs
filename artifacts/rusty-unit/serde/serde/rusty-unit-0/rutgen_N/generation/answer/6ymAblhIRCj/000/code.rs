// Answer 0

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn bad_type(_: Unsupported) -> String {
        "bad type".to_string()
    }

    fn serialize_u8(self, _: u8) -> Result<(), String> {
        Err(Self::bad_type(Unsupported::Integer))
    }
}

#[derive(Debug)]
struct Unsupported {
    kind: String,
}

impl Unsupported {
    const Integer: Unsupported = Unsupported { kind: "Integer".to_string() };
}

#[test]
fn test_serialize_u8_should_return_error() {
    let serializer = Serializer;
    let result = serializer.serialize_u8(5);
    assert_eq!(result, Err("bad type".to_string()));
}

