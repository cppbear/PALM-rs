// Answer 0

#[derive(Debug)]
struct TestDeserializer;

impl TestDeserializer {
    fn into_deserializer(self) -> Self {
        self
    }
}

#[test]
fn test_into_deserializer() {
    let deserializer = TestDeserializer;
    let result = deserializer.into_deserializer();
    assert_eq!(format!("{:?}", result), format!("{:?}", deserializer));
}

