// Answer 0

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn bad_type(&self, _: Unsupported) -> Result<(), ()> {
        Err(())
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<(), ()> {
        Err(self.bad_type(Unsupported::ByteArray))
    }
}

#[derive(Debug)]
struct Unsupported;

impl Unsupported {
    const ByteArray: Self = Unsupported;
}

#[test]
fn test_serialize_bytes_should_return_error() {
    let serializer = Serializer;
    let result = serializer.serialize_bytes(&[1, 2, 3]);
    assert!(result.is_err());
}

