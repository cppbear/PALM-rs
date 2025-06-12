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
    let input = TestStruct;
    let result = input.into_deserializer();
    assert_eq!(format!("{:?}", result), format!("{:?}", input));
}

#[test]
fn test_into_deserializer_empty() {
    let input = TestStruct;
    let result = input.into_deserializer();
    assert_eq!(result, input);
}

