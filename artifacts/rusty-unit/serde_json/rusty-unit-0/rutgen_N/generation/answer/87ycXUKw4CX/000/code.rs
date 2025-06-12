// Answer 0

#[derive(Debug)]
struct MyDeserializer;

impl MyDeserializer {
    fn into_deserializer(self) -> Self {
        self
    }
}

#[test]
fn test_into_deserializer() {
    let deserializer = MyDeserializer;
    let result = deserializer.into_deserializer();
    assert_eq!(format!("{:?}", deserializer), format!("{:?}", result));
}

