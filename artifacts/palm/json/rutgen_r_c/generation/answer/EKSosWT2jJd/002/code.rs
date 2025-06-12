// Answer 0

#[test]
fn test_unit_variant_with_none() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char string bytes byte_buf
            option seq map struct enum identifier newtype_struct
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an empty variant")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(())
        }
    }

    let deserializer = VariantRefDeserializer { value: None };

    let result: Result<(), Error> = deserializer.unit_variant();
    assert!(result.is_ok());
}

