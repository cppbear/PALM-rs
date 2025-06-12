// Answer 0

#[derive(Debug)]
struct Serializer;

#[derive(Debug)]
struct Error;

trait Serialize {}

impl Serializer {
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = value;
        Err(Error)
    }
}

struct SerializableStruct;

impl Serialize for SerializableStruct {}

#[test]
fn test_serialize_element() {
    let mut serializer = Serializer;
    let value = SerializableStruct;

    let result = serializer.serialize_element(&value);
    assert!(result.is_err());
}

