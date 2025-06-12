// Answer 0

#[derive(Debug)]
struct Serialize;

impl Serialize {
    fn bad_type(_: Unsupported) -> &'static str {
        "bad type"
    }
}

struct Unsupported {
    _type: &'static str,
}

impl Unsupported {
    const Integer: &'static str = "Integer";
}

impl Serialize {
    fn serialize_i64(self, _: i64) -> Result<&'static str, &'static str> {
        Err(Self::bad_type(Unsupported { _type: Self::Integer }))
    }
}

#[test]
fn test_serialize_i64_returns_error() {
    let serializer = Serialize;

    let result = serializer.serialize_i64(42);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "bad type");
}

