// Answer 0

#[test]
fn test_visit_enum_success() {
    struct TestEnumAccess;

    impl<'de> EnumAccess<'de> for TestEnumAccess {
        type Error = ();
        type Variants = TestVariants;

        fn variant<V>(self) -> Result<(V, Self::Variants), Self::Error> where V: Deserialize<'de> {
            Ok((IgnoredAny, TestVariants))
        }
    }

    struct TestVariants;

    impl<'de> crate::de::VariantAccess<'de> for TestVariants {
        type Error = ();

        fn newtype_variant<D>(self) -> Result<IgnoredAny, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(IgnoredAny)
        }
    }

    let access = TestEnumAccess;
    let result: Result<IgnoredAny, ()> = IgnoredAny.visit_enum(access);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_visit_enum_failure_variant() {
    struct FailingEnumAccess;

    impl<'de> EnumAccess<'de> for FailingEnumAccess {
        type Error = ();
        type Variants = ();

        fn variant<V>(self) -> Result<(V, Self::Variants), Self::Error>
        where
            V: Deserialize<'de>
        {
            Err(())
        }
    }

    let access = FailingEnumAccess;
    let _: Result<IgnoredAny, ()> = IgnoredAny.visit_enum(access);
}

