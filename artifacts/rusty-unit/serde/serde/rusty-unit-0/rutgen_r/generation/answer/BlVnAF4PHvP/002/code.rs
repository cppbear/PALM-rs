// Answer 0

#[derive(Debug)]
struct DummyDeserializer<'de>(&'de [&'de str]);

impl<'de> DummyDeserializer<'de> {
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, String>
    where
        V: Visitor<'de>,
    {
        for entry in self.0 {
            if let Some((key, value)) = flat_map_take_entry(entry, variants) {
                return visitor.visit_enum(EnumDeserializer::new(key, Some(value)));
            }
        }

        Err(format!("no variant of enum {} found in flattened data", name))
    }
}

fn flat_map_take_entry(entry: &str, variants: &'static [&'static str]) -> Option<(&'static str, &'static str)> {
    if variants.contains(&entry) {
        Some((entry, entry))
    } else {
        None
    }
}

struct EnumDeserializer<'de> {
    key: &'static str,
    value: Option<&'static str>,
}

impl<'de> EnumDeserializer<'de> {
    fn new(key: &'static str, value: Option<&'static str>) -> Self {
        Self { key, value }
    }
}

trait Visitor<'de> {
    type Value;

    fn visit_enum(self, deserializer: EnumDeserializer<'de>) -> Result<Self::Value, String>;
}

struct DummyVisitor;

impl<'de> Visitor<'de> for DummyVisitor {
    type Value = &'static str;

    fn visit_enum(self, deserializer: EnumDeserializer<'de>) -> Result<Self::Value, String> {
        Ok(deserializer.key)
    }
}

#[test]
fn test_deserialize_enum_success() {
    let variants = &["variant1", "variant2", "variant3"];
    let entries = &["variant1", "not_a_variant", "variant2"];
    let deserializer = DummyDeserializer(entries);
    let visitor = DummyVisitor;

    let result = deserializer.deserialize_enum("TestEnum", variants, visitor).unwrap();
    
    assert_eq!(result, "variant1");
}

#[test]
fn test_deserialize_enum_no_variant_found() {
    let variants = &["variant1", "variant2", "variant3"];
    let entries = &["not_a_variant"];
    let deserializer = DummyDeserializer(entries);
    let visitor = DummyVisitor;

    let result = deserializer.deserialize_enum("TestEnum", variants, visitor);
    
    assert_eq!(result.unwrap_err(), "no variant of enum TestEnum found in flattened data");
}

