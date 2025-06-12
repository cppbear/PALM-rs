// Answer 0

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn serialize_none(self) -> Result<(), &'static str> {
        Ok(())
    }
}

#[test]
fn test_serialize_none_success() {
    let serializer = Serializer;
    let result = serializer.serialize_none();
    assert_eq!(result, Ok(()));
}

