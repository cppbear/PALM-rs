// Answer 0

#[derive(Debug)]
struct FlatMapSerializeStruct(i32);

struct SerializeStructWrapper(i32);

impl SerializeStructWrapper {
    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<FlatMapSerializeStruct, String> {
        Ok(FlatMapSerializeStruct(self.0))
    }
}

#[test]
fn test_serialize_struct() {
    let wrapper = SerializeStructWrapper(42);
    let result = wrapper.serialize_struct("test", 0);
    assert!(result.is_ok());
    let serialize_struct = result.unwrap();
    assert_eq!(serialize_struct.0, 42);
}

#[test]
fn test_serialize_struct_with_different_values() {
    let wrapper = SerializeStructWrapper(100);
    let result = wrapper.serialize_struct("test", 1);
    assert!(result.is_ok());
    let serialize_struct = result.unwrap();
    assert_eq!(serialize_struct.0, 100);
}

