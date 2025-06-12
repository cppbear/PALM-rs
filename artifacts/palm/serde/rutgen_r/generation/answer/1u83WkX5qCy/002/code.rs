// Answer 0

#[derive(Debug)]
struct MockDelegate;

impl MockDelegate {
    fn serialize_map(&self, _: Option<usize>) -> Result<MockMap, String> {
        Ok(MockMap)
    }
}

struct MockMap;

impl MockMap {
    fn serialize_entry(&mut self, _: &'static str, _: &'static str) -> Result<(), String> {
        Err("Serialization error".to_string())
    }

    fn end(self) -> Result<(), String> {
        Ok(())
    }
}

struct Serializer {
    delegate: MockDelegate,
    tag: &'static str,
    variant_name: &'static str,
}

impl Serializer {
    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
        inner_value: &T,
    ) -> Result<(), String>
    where
        T: ?Sized + serde::ser::Serialize,
    {
        let mut map = self.delegate.serialize_map(Some(2))?;
        map.serialize_entry(self.tag, self.variant_name)?;
        map.serialize_entry(inner_variant, inner_value)?;
        map.end()
    }
}

#[test]
fn test_serialize_newtype_variant_panics_on_entry_fail() {
    let serializer = Serializer {
        delegate: MockDelegate,
        tag: "tag",
        variant_name: "variant_name",
    };
    let result = serializer.serialize_newtype_variant("newtype", 0, "inner_variant", &"inner_value");
    assert!(result.is_err());
}

