// Answer 0

#[test]
fn test_visit_enum_success() {
    struct TestEnumAccess;
    impl<'de> EnumAccess<'de> for TestEnumAccess {
        type Error = ();
        type Variant = TestVariantAccess;

        fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>
        where
            V: Visitor<'de>,
        {
            Ok((IgnoredAny, TestVariantAccess))
        }
    }

    struct TestVariantAccess;

    impl<'de> VariantAccess<'de> for TestVariantAccess {
        type Error = ();
        
        fn unit_variant(self) -> Result<IgnoredAny, Self::Error> {
            Ok(IgnoredAny)
        }

        fn newtype_variant<D>(self, _: D) -> Result<IgnoredAny, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(IgnoredAny)
        }

        fn tuple_variant<D>(self, _: D, _: usize) -> Result<IgnoredAny, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(IgnoredAny)
        }

        fn struct_variant<D>(self, _: D, _: &'static [&'static str]) -> Result<IgnoredAny, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(IgnoredAny)
        }
    }

    let visitor = IgnoredAny;
    let result = visitor.visit_enum(TestEnumAccess);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_visit_enum_failure() {
    struct FailingEnumAccess;
    impl<'de> EnumAccess<'de> for FailingEnumAccess {
        type Error = ();
        type Variant = FailingVariantAccess;

        fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>
        where
            V: Visitor<'de>,
        {
            Err(())
        }
    }

    struct FailingVariantAccess;

    impl<'de> VariantAccess<'de> for FailingVariantAccess {
        type Error = ();
        
        fn unit_variant(self) -> Result<IgnoredAny, Self::Error> {
            Err(())
        }

        fn newtype_variant<D>(self, _: D) -> Result<IgnoredAny, D::Error>
        where
            D: Deserializer<'de>,
        {
            Err(())
        }

        fn tuple_variant<D>(self, _: D, _: usize) -> Result<IgnoredAny, D::Error>
        where
            D: Deserializer<'de>,
        {
            Err(())
        }

        fn struct_variant<D>(self, _: D, _: &'static [&'static str]) -> Result<IgnoredAny, D::Error>
        where
            D: Deserializer<'de>,
        {
            Err(())
        }
    }

    let visitor = IgnoredAny;
    let _ = visitor.visit_enum(FailingEnumAccess);
}

