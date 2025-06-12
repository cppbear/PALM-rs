// Answer 0

#[derive(Debug)]
struct MyDeserializer;

impl MyDeserializer {
    fn into_deserializer(self) -> Self {
        self
    }
}

#[test]
fn test_into_deserializer_returns_self() {
    let deserializer = MyDeserializer;
    let returned = deserializer.into_deserializer();
    assert_eq!(format!("{:?}", returned), format!("{:?}", deserializer));
}

