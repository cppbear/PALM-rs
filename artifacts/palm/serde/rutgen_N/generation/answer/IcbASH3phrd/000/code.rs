// Answer 0

#[derive(Debug)]
struct MyDeserializer;

impl MyDeserializer {
    fn into_deserializer(self) -> Self {
        self
    }
}

#[test]
fn test_into_deserializer_basic() {
    let deserializer = MyDeserializer;
    let result = deserializer.into_deserializer();
    assert_eq!(format!("{:?}", result), format!("{:?}", deserializer));
}

#[test]
fn test_into_deserializer_multiple_calls() {
    let deserializer = MyDeserializer;
    let result = deserializer.into_deserializer().into_deserializer();
    assert_eq!(format!("{:?}", result), format!("{:?}", deserializer));
}

