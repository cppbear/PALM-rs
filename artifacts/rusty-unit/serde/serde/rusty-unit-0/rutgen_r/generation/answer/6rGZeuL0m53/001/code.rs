// Answer 0

#[derive(Debug)]
struct MockSerializer;

impl MockSerializer {
    fn bad_type(&self, _: Unsupported) -> &'static str {
        "Error: Unsupported type"
    }

    fn serialize_bool(self, _: bool) -> Result<&'static str, &'static str> {
        Err(self.bad_type(Unsupported::Boolean))
    }
}

#[derive(Debug)]
struct Unsupported;

impl Unsupported {
    const Boolean: Self = Self;
}

#[test]
fn test_serialize_bool_returns_err_for_boolean() {
    let serializer = MockSerializer;
    let result = serializer.serialize_bool(true);
    assert_eq!(result.unwrap_err(), "Error: Unsupported type");
}

#[test]
fn test_serialize_bool_returns_err_for_false() {
    let serializer = MockSerializer;
    let result = serializer.serialize_bool(false);
    assert_eq!(result.unwrap_err(), "Error: Unsupported type");
}

