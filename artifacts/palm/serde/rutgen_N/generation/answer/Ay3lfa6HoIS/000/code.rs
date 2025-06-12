// Answer 0

#[derive(Debug)]
struct SerdeSerializer;

impl SerdeSerializer {
    fn bad_type(&self, _: Unsupported) -> Result<(), &'static str> {
        Err("Unsupported type")
    }

    fn serialize_i16(self, _: i16) -> Result<(), &'static str> {
        Err(self.bad_type(Unsupported::Integer))
    }
}

#[derive(Debug)]
struct Unsupported;

impl Unsupported {
    const Integer: Self = Unsupported;
}

#[test]
fn test_serialize_i16_unsupported_type() {
    let serializer = SerdeSerializer;
    let result = serializer.serialize_i16(16);
    assert_eq!(result, Err("Unsupported type"));
}

