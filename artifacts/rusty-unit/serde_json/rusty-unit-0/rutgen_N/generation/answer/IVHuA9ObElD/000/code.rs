// Answer 0

#[derive(Debug)]
struct MyStruct;

impl MyStruct {
    fn into_deserializer(self) -> Self {
        self
    }
}

#[test]
fn test_into_deserializer() {
    let my_struct = MyStruct;
    let deserializer = my_struct.into_deserializer();
    assert_eq!(format!("{:?}", deserializer), format!("{:?}", my_struct));
}

