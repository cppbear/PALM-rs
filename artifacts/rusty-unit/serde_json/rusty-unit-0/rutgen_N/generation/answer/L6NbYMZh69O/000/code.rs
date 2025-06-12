// Answer 0

#[derive(Debug)]
struct TestStruct;

impl TestStruct {
    fn into_deserializer(self) -> TestStruct {
        self
    }
}

#[test]
fn test_into_deserializer() {
    let instance = TestStruct;
    let deserializer = instance.into_deserializer();
    assert_eq!(format!("{:?}", deserializer), format!("{:?}", instance));
}

