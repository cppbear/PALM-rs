// Answer 0

#[derive(Default)]
struct TestSerializer;

impl TestSerializer {
    fn serialize_key(&self, key: &'static str) -> Result<(), &'static str> {
        if key.is_empty() {
            Err("Key cannot be empty")
        } else {
            Ok(())
        }
    }
}

struct SerializeStructVariantHelper<'a>(&'a TestSerializer);

impl SerializeStructVariantHelper<'_> {
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
        _: usize,
    ) -> Result<FlatMapSerializeStructVariantAsMapValue, &'static str> {
        self.0.serialize_key(inner_variant)?;
        Ok(FlatMapSerializeStructVariantAsMapValue::new(self.0, inner_variant))
    }
}

struct FlatMapSerializeStructVariantAsMapValue<'a> {
    serializer: &'a TestSerializer,
    inner_variant: &'static str,
}

impl<'a> FlatMapSerializeStructVariantAsMapValue<'a> {
    fn new(serializer: &'a TestSerializer, inner_variant: &'static str) -> Self {
        FlatMapSerializeStructVariantAsMapValue {
            serializer,
            inner_variant,
        }
    }
}

#[test]
fn test_serialize_struct_variant_success() {
    let serializer = TestSerializer::default();
    let helper = SerializeStructVariantHelper(&serializer);
    
    let result = helper.serialize_struct_variant("root", 0, "inner_variant", 0);
    
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Key cannot be empty")]
fn test_serialize_struct_variant_empty_key() {
    let serializer = TestSerializer::default();
    let helper = SerializeStructVariantHelper(&serializer);
    
    let _ = helper.serialize_struct_variant("root", 0, "", 0).unwrap();
}

