// Answer 0

#[derive(Debug)]
struct MyStruct;

impl MyStruct {
    fn into_deserializer(self) -> MyStruct {
        self
    }
}

#[test]
fn test_into_deserializer() {
    let instance = MyStruct;
    let result = instance.into_deserializer();
    assert_eq!(format!("{:?}", result), "MyStruct");
}

#[test]
fn test_into_deserializer_identity() {
    let instance = MyStruct;
    let result = instance.into_deserializer();
    assert!(std::ptr::eq(&instance, &result));
}

