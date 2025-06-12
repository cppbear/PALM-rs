// Answer 0

#[test]
fn test_deserialize_enum_invalid_type() {
    struct FakeVisitor;

    impl<'de> Visitor<'de> for FakeVisitor {
        type Value = ();
        
        fn visit_enum<V>(self, _deserializer: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::EnumAccess<'de>,
        {
            Ok(())
        }
    }
    
    enum Content {
        Map(std::collections::HashMap<String, String>),
        String(String),
        Str(&'static str),
        Other,
    }

    struct Deserializer {
        content: Content,
    }

    impl Deserializer {
        fn deserialize_enum<V>(
            self,
            _name: &str,
            _variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, String>
        where
            V: Visitor<'de>,
        {
            let (variant, value) = match self.content {
                Content::Map(value) => {
                    let mut iter = value.into_iter();
                    let (variant, value) = match iter.next() {
                        Some(v) => v,
                        None => {
                            return Err("invalid_value".into());
                        }
                    };
                    if iter.next().is_some() {
                        return Err("invalid_value".into());
                    }
                    (variant, Some(value))
                }
                s @ Content::String(_) | s @ Content::Str(_) => (s, None),
                other => {
                    return Err("invalid_type".into());
                }
            };

            visitor.visit_enum(variant)
        }
    }

    let deserializer = Deserializer { content: Content::Other };
    let visitor = FakeVisitor;

    let result: Result<_, String> = deserializer.deserialize_enum("enum_name", &[], visitor);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "invalid_type");
}

