// Answer 0

#[test]
fn test_deserialize_enum_valid() {
    struct MockVisitor {
        value: Option<String>,
    }
    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            formatter.write_str("a valid enum variant")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    struct MockDeserializer {
        de: Deserializer<SliceRead<'static>>,
    }
    
    impl<'de> de::Deserializer<'de> for MockDeserializer {
        type Error = Error;

        fn deserialize_enum<V>(
            self,
            name: &'static str,
            variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            if variants.contains(&"Variant1") {
                visitor.visit_str("Variant1")
            } else {
                Err(Error)
            }
        }
    }

    let deserializer = MockDeserializer {
        de: Deserializer {
            read: SliceRead::new(b""),
            scratch: Vec::new(),
            remaining_depth: 0,
        },
    };

    let result = deserializer.deserialize_enum("MockEnum", &["Variant1", "Variant2"], MockVisitor { value: None }).unwrap();
    assert_eq!(result, "Variant1");
}

#[test]
#[should_panic]
fn test_deserialize_enum_invalid_variant() {
    struct MockVisitor {
        value: Option<String>,
    }
    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = String;
        
        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            formatter.write_str("a valid enum variant")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value>
        where
            E: de::Error,
        {
            panic!("Unexpected visit_str: {}", value);
        }
    }

    struct MockDeserializer {
        de: Deserializer<SliceRead<'static>>,
    }
    
    impl<'de> de::Deserializer<'de> for MockDeserializer {
        type Error = Error;

        fn deserialize_enum<V>(
            self,
            _name: &'static str,
            variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            if !variants.contains(&"InvalidVariant") {
                return Err(Error);
            }
            visitor.visit_str("InvalidVariant")
        }
    }

    let deserializer = MockDeserializer {
        de: Deserializer {
            read: SliceRead::new(b""),
            scratch: Vec::new(),
            remaining_depth: 0,
        },
    };

    deserializer.deserialize_enum("MockEnum", &["Variant1", "Variant2"], MockVisitor { value: None }).unwrap();
}

