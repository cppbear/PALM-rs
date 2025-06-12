// Answer 0

#[derive(Debug)]
struct MyStruct;

impl MyStruct {
    fn into_deserializer(self) -> MyDeserializer {
        MyDeserializer
    }
}

struct MyDeserializer;

impl MyStruct {
    fn from(self) -> MyDeserializer {
        self.into_deserializer()
    }
}

#[test]
fn test_from() {
    let my_struct = MyStruct;
    let deserializer = my_struct.from();
    assert!(std::any::TypeId::of::<MyDeserializer>() == std::any::TypeId::of_val(&deserializer));
}

