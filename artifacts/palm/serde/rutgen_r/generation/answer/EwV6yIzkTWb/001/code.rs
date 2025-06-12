// Answer 0

#[test]
fn test_serialize_struct_success() {
    struct SerializeStruct<'a> {
        name: &'a str,
        fields: Vec<String>,
        error: std::marker::PhantomData<*const ()>,
    }
    
    struct Serializer;

    impl Serializer {
        fn serialize_struct(self, name: &'static str, len: usize) -> Result<SerializeStruct<'static>, &'static str> {
            Ok(SerializeStruct {
                name,
                fields: Vec::with_capacity(len),
                error: std::marker::PhantomData,
            })
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_struct("TestStruct", 10);
    assert!(result.is_ok());
    
    let serialize_struct = result.unwrap();
    assert_eq!(serialize_struct.name, "TestStruct");
    assert_eq!(serialize_struct.fields.capacity(), 10);
}

#[test]
fn test_serialize_struct_zero_capacity() {
    struct SerializeStruct<'a> {
        name: &'a str,
        fields: Vec<String>,
        error: std::marker::PhantomData<*const ()>,
    }
    
    struct Serializer;

    impl Serializer {
        fn serialize_struct(self, name: &'static str, len: usize) -> Result<SerializeStruct<'static>, &'static str> {
            Ok(SerializeStruct {
                name,
                fields: Vec::with_capacity(len),
                error: std::marker::PhantomData,
            })
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_struct("EmptyStruct", 0);
    assert!(result.is_ok());

    let serialize_struct = result.unwrap();
    assert_eq!(serialize_struct.name, "EmptyStruct");
    assert_eq!(serialize_struct.fields.capacity(), 0);
}

