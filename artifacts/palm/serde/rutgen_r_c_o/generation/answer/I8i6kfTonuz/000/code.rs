// Answer 0

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn bad_type(_: Unsupported) -> ! {
        panic!("bad type")
    }

    fn serialize_bool(self, _: bool) -> Result<(), String> {
        Err(Self::bad_type(Unsupported::Boolean).to_string())
    }
}

#[derive(Debug)]
enum Unsupported {
    Boolean,
}

#[test]
fn test_serialize_bool_should_return_error() {
    let serializer = Serializer;
    let result = serializer.serialize_bool(true);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "bad type");
}

