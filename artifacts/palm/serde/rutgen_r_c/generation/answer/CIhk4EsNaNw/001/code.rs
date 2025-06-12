// Answer 0

#[test]
fn test_visit_enum_valid() {
    struct TestEnumVisitor;

    impl<'de> EnumAccess<'de> for TestEnumVisitor {
        type Error = serde::de::Error;
        type Variant = TestVariantVisitor;

        fn variant_seed<V>(self, seed: V) -> Result<(Self::Variant, V::Value), Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            let value = seed.deserialize(TestVariantDeserializer)?;
            Ok((TestVariantVisitor, value))
        }
    }

    struct TestVariantVisitor;

    impl<'de> Visitor<'de> for TestVariantVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a test variant")
        }
        
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(())
        }
    }

    let visitor = TagOrContentVisitor {
        name: "MyEnum",
        value: PhantomData,
    };
    
    let result = visitor.visit_enum(TestEnumVisitor);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "invalid type")]
fn test_visit_enum_invalid() {
    struct InvalidVisitor;

    impl<'de> EnumAccess<'de> for InvalidVisitor {
        type Error = serde::de::Error;
        type Variant = ();

        fn variant_seed<V>(self, seed: V) -> Result<(Self::Variant, V::Value), Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            Err(serde::de::Error::custom("invalid visitor"))
        }
    }

    let visitor = TagOrContentVisitor {
        name: "MyEnum",
        value: PhantomData,
    };
    
    let _ = visitor.visit_enum(InvalidVisitor);
}

