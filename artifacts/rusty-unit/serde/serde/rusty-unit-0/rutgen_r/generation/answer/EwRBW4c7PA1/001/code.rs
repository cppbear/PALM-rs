// Answer 0

#[derive(Debug)]
struct MockDelegate;

impl MockDelegate {
    fn serialize_map(&self, _: Option<usize>) -> Result<MockMap, String> {
        Err("Serialization error".to_string())
    }
}

struct MockMap;

impl MockMap {
    fn serialize_entry(&mut self, _: &str, _: &str) -> Result<(), String> {
        Ok(())
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
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), String> {
        let mut map = self.delegate.serialize_map(Some(1))?;
        map.serialize_entry(self.tag, self.variant_name)?;
        map.end()
    }
}

#[test]
fn test_serialize_unit_struct_returns_error_on_serialize_map_failure() {
    let serializer = Serializer {
        delegate: MockDelegate,
        tag: "tag",
        variant_name: "variant",
    };
    let result = serializer.serialize_unit_struct("unit_struct");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Serialization error");
}

