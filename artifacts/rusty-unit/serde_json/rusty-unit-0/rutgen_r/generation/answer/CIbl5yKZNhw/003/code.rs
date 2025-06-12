// Answer 0

#[test]
fn test_deserialize_enum_empty_map() {
    use serde::de::{self, Visitor, Unexpected};
    use std::marker::PhantomData;
    use serde_json::Value;

    struct DummyVisitor {
        marker: PhantomData<Value>,
    }

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = Result<Value, de::Error>;

        fn visit_enum<V>(self, _value: V) -> Self::Value {
            Ok(Value::Null)
        }
    }

    struct DummyDeserializer {
        iter: std::slice::Iter<'static, (&'static str, Value)>,
    }

    impl DummyDeserializer {
        fn new() -> Self {
            DummyDeserializer {
                iter: [].iter(),
            }
        }
    }

    impl<'de> serde::Deserializer<'de> for DummyDeserializer {
        type Error = de::Error;

        fn deserialize_enum<V>(
            self,
            _name: &'static str,
            _variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let mut iter = self.iter;
            let (variant, value) = match iter.next() {
                Some(v) => v,
                None => {
                    return Err(de::Error::invalid_value(
                        Unexpected::Map,
                        &"map with a single key",
                    ));
                }
            };
            if iter.next().is_some() {
                return Err(de::Error::invalid_value(
                    Unexpected::Map,
                    &"map with a single key",
                ));
            }

            visitor.visit_enum((variant, value))
        }

        // Other methods of Deserializer are omitted for brevity
    }

    let deserializer = DummyDeserializer::new();
    let result: Result<Value, _> = deserializer.deserialize_enum("TestEnum", &["Variant1", "Variant2"], DummyVisitor { marker: PhantomData });

    assert!(result.is_err());
    if let Err(e) = result {
        match e {
            de::Error::InvalidValue { .. } => {}
            _ => panic!("Expected InvalidValue error"),
        }
    }
}

