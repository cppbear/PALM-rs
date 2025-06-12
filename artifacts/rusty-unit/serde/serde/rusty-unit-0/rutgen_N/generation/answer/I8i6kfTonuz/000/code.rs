// Answer 0

#[derive(Debug)]
struct MockSerializer;

impl MockSerializer {
    fn bad_type(_: Unsupported) -> serde::ser::Error {
        serde::ser::Error::custom("unsupported type")
    }
}

trait Serializer {
    type Ok;
    type Error;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error>;
}

impl Serializer for MockSerializer {
    type Ok = ();
    type Error = serde::ser::Error;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Boolean))
    }
}

#[test]
fn test_serialize_bool_returns_error() {
    let serializer = MockSerializer;
    let result = serializer.serialize_bool(true);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "unsupported type");
}

