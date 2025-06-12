// Answer 0

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn bad_type(&self, _: Unsupported) -> Result<(), ()> {
        Err(())
    }

    fn serialize_i64(self, _: i64) -> Result<(), ()> {
        Err(self.bad_type(Unsupported::Integer))
    }
}

#[derive(Debug)]
struct Unsupported;

impl Unsupported {
    const Integer: Self = Unsupported;
}

#[test]
fn test_serialize_i64_returns_err() {
    let serializer = Serializer;
    let result = serializer.serialize_i64(123456789);
    assert!(result.is_err());
}

