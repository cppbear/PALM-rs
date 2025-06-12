// Answer 0

#[derive(Debug)]
struct SerializeStruct<'a, E> {
    name: &'a str,
    fields: Vec<String>,
    error: std::marker::PhantomData<E>,
}

struct Serializer;

impl Serializer {
    fn serialize_struct<E>(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<SerializeStruct<'static, E>, E> {
        Ok(SerializeStruct {
            name,
            fields: Vec::with_capacity(len),
            error: std::marker::PhantomData,
        })
    }
}

#[test]
fn test_serialize_struct_creates_struct() {
    let serializer = Serializer;

    let result: Result<SerializeStruct<'static, ()>, ()> = serializer.serialize_struct("TestStruct", 3);

    assert!(result.is_ok());
    let struct_instance = result.unwrap();
    assert_eq!(struct_instance.name, "TestStruct");
    assert_eq!(struct_instance.fields.capacity(), 3);
}

#[test]
fn test_serialize_struct_zero_length() {
    let serializer = Serializer;

    let result: Result<SerializeStruct<'static, ()>, ()> = serializer.serialize_struct("EmptyStruct", 0);

    assert!(result.is_ok());
    let struct_instance = result.unwrap();
    assert_eq!(struct_instance.name, "EmptyStruct");
    assert_eq!(struct_instance.fields.capacity(), 0);
}

