// Answer 0

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn bad_type(&self, _: Unsupported) -> ! {
        panic!("Bad type")
    }
}

#[derive(Debug)]
enum Unsupported {
    Float,
}

impl Serializer {
    fn serialize_f32(self, _: f32) -> Result<(), ()> {
        Err(self.bad_type(Unsupported::Float))
    }
}

#[test]
fn test_serialize_f32_should_return_error() {
    let serializer = Serializer;
    let result = serializer.serialize_f32(3.14);
    assert!(result.is_err());
}

