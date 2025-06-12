// Answer 0

#[derive(Debug)]
struct MockVisitor {
    value: Option<String>,
}

impl<'de> Visitor<'de> for MockVisitor {
    type Value = String;

    fn visit_enum<E>(self, _deserializer: EnumDeserializer<'de, E>) -> Result<Self::Value, E> {
        Ok(self.value.unwrap_or_else(|| "default".to_string()))
    }
}

#[derive(Debug)]
enum Content<'de> {
    Map(Vec<(&'de str, Option<&'de str>)>),
    String(&'de str),
    Str(&'de str),
}

struct Deserializer<'de> {
    content: Content<'de>,
}

impl<'de> Deserializer<'de> {
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, &'static str>
    where
        V: Visitor<'de>,
    {
        let (variant, value) = match self.content {
            Content::Map(value) => {
                let mut iter = value.into_iter();
                let (variant, value) = match iter.next() {
                    Some(v) => v,
                    None => {
                        return Err("invalid_value: map with a single key");
                    }
                };

                if iter.next().is_some() {
                    return Err("invalid_value: map with a single key");
                }
                (variant, value)
            }
            s @ Content::String(_) | s @ Content::Str(_) => (s, None),
            _ => {
                return Err("invalid_type: string or map");
            }
        };

        visitor.visit_enum(EnumDeserializer::new(variant, value))
    }
}

struct EnumDeserializer<'de, E> {
    variant: Content<'de>,
    value: Option<&'de str>,
}

impl<'de, E> EnumDeserializer<'de, E> {
    fn new(variant: Content<'de>, value: Option<&'de str>) -> Self {
        EnumDeserializer { variant, value }
    }
}

#[test]
fn test_deserialize_enum_with_string_content() {
    let deserializer = Deserializer {
        content: Content::Str("example_variant"),
    };

    let visitor = MockVisitor { value: None };

    let result = deserializer.deserialize_enum("enum_name", &["example_variant"], visitor);

    assert_eq!(result.unwrap(), "default");
}

#[test]
fn test_deserialize_enum_with_map_content() {
    let deserializer = Deserializer {
        content: Content::Map(vec![("example_variant", Some("value"))]),
    };

    let visitor = MockVisitor { value: Some("value".to_string()) };

    let result = deserializer.deserialize_enum("enum_name", &["example_variant"], visitor);

    assert_eq!(result.unwrap(), "value");
}

