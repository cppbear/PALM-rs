// Answer 0

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn bad_type(&self, _: Unsupported) -> Result<(), ()> {
        Err(())
    }

    fn serialize_char(self, _: char) -> Result<(), ()> {
        Err(self.bad_type(Unsupported::Char))
    }
}

#[derive(Debug)]
struct Unsupported;

impl Unsupported {
    const Char: Self = Self;
}

#[test]
fn test_serialize_char_returns_error() {
    let serializer = Serializer;
    let result = serializer.serialize_char('a');
    assert!(result.is_err());
}

