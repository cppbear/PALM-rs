// Answer 0

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn bad_type(_: Unsupported) -> String {
        "bad type".to_string()
    }
}

#[derive(Debug)]
struct Unsupported {
    kind: String,
}

impl Unsupported {
    const Integer: Self = Unsupported {
        kind: "Integer".to_string(),
    };
}

impl Serializer {
    type Ok = ();
    type Error = String;

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }
}

#[test]
fn test_serialize_u32_returns_error() {
    let serializer = Serializer;
    let result = serializer.serialize_u32(42);
    assert_eq!(result, Err("bad type".to_string()));
}

