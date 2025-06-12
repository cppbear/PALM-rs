// Answer 0

#[derive(Debug)]
struct TestSerializer;

impl TestSerializer {
    fn serialize_key(&self, _key: &'static str) -> Result<(), &'static str> {
        Ok(())
    }
}

struct FlatMapSerializeTupleVariantAsMapValue<'a>(&'a TestSerializer);

impl<'a> FlatMapSerializeTupleVariantAsMapValue<'a> {
    fn new(serializer: &'a TestSerializer) -> Self {
        FlatMapSerializeTupleVariantAsMapValue(serializer)
    }
}

struct TestSerializableTupleVariant<'a>(&'a TestSerializer);

impl<'a> TestSerializableTupleVariant<'a> {
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<FlatMapSerializeTupleVariantAsMapValue<'a>, &'static str> {
        self.0.serialize_key(variant)?;
        Ok(FlatMapSerializeTupleVariantAsMapValue::new(self.0))
    }
}

#[test]
fn test_serialize_tuple_variant() {
    let serializer = TestSerializer;
    let serializable_tuple_variant = TestSerializableTupleVariant(&serializer);

    let result = serializable_tuple_variant.serialize_tuple_variant("type_name", 0, "variant_name", 0);
    
    assert!(result.is_ok());
    let _variant = result.unwrap();
}

