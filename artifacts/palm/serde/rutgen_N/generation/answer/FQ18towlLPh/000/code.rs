// Answer 0

#[test]
fn test_serialize_struct_variant() {
    struct SerializeStructVariant<'a> {
        name: &'a str,
        variant_index: u32,
        variant: &'a str,
        fields: Vec<()>,
        error: std::marker::PhantomData<()>,
    }

    struct Serializer;

    impl Serializer {
        fn serialize_struct_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<SerializeStructVariant, &'static str> {
            Ok(SerializeStructVariant {
                name,
                variant_index,
                variant,
                fields: Vec::with_capacity(len),
                error: std::marker::PhantomData,
            })
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_struct_variant("MyStruct", 0, "MyVariant", 2);

    assert!(result.is_ok());
    let variant = result.unwrap();
    assert_eq!(variant.name, "MyStruct");
    assert_eq!(variant.variant_index, 0);
    assert_eq!(variant.variant, "MyVariant");
    assert_eq!(variant.fields.capacity(), 2);
}

