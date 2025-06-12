// Answer 0

#[derive(Default)]
struct MockDelegate {
    fields: Vec<(String, String)>,
}

impl MockDelegate {
    fn serialize_struct(&mut self, _name: &'static str, len: usize) -> Result<MockSerializeStruct, &'static str> {
        Ok(MockSerializeStruct { fields: vec![], expected_len: len })
    }
}

struct MockSerializeStruct {
    fields: Vec<(String, String)>,
    expected_len: usize,
}

impl MockSerializeStruct {
    fn serialize_field(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
        if self.fields.len() < self.expected_len {
            self.fields.push((key.to_string(), value.to_string()));
            Ok(())
        } else {
            Err("Exceeded the expected length")
        }
    }
}

struct Serializer {
    delegate: MockDelegate,
    tag: &'static str,
    variant_name: &'static str,
}

impl Serializer {
    fn new(tag: &'static str, variant_name: &'static str) -> Self {
        Serializer {
            delegate: MockDelegate::default(),
            tag,
            variant_name,
        }
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<MockSerializeStruct, &'static str> {
        let mut state = self.delegate.serialize_struct(name, len + 1)?;
        state.serialize_field(self.tag, self.variant_name)?;
        Ok(state)
    }
}

#[test]
fn test_serialize_struct_success() {
    let serializer = Serializer::new("test_tag", "test_variant");
    let result = serializer.serialize_struct("test_name", 1);
    assert!(result.is_ok());
    let state = result.unwrap();
    assert_eq!(state.fields.len(), 1);
    assert_eq!(state.fields[0], ("test_tag".to_string(), "test_variant".to_string()));
}

#[test]
fn test_serialize_struct_exceed_length() {
    let serializer = Serializer::new("test_tag", "test_variant");
    let result = serializer.serialize_struct("test_name", 0);
    assert!(result.is_ok());
    let mut state = result.unwrap();
    let _ = state.serialize_field("extra_field", "value");
    let err = state.serialize_field("extra_field2", "value2").unwrap_err();
    assert_eq!(err, "Exceeded the expected length");
}

