// Answer 0

#[derive(Debug)]
struct TestStruct;

impl TestStruct {
    fn into_deserializer(self) -> Self {
        self
    }
}

#[test]
fn test_into_deserializer() {
    let test_instance = TestStruct;
    let result = test_instance.into_deserializer();
    assert_eq!(format!("{:?}", test_instance), format!("{:?}", result));
}

