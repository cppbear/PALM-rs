// Answer 0

#[test]
fn test_visit_enum_valid_variant() {
    struct ValidEnum;
    enum TestEnum {
        Variant1(IgnoredAny),
        Variant2(IgnoredAny),
    }

    impl<'de> EnumAccess<'de> for ValidEnum {
        type Error = serde::de::Error;
        type Variant = ValidVariant;

        fn variant<T>(self) -> Result<(T, Self::Variant), Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok((IgnoredAny, ValidVariant))
        }
    }

    struct ValidVariant;
    
    impl VariantAccess<'de> for ValidVariant {
        type Error = serde::de::Error;

        fn newtype_variant<T>(self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(IgnoredAny)
        }
    }

    let visitor = IgnoredAny;
    let enum_access = ValidEnum;

    let _ = visitor.visit_enum(enum_access);
}

#[test]
fn test_visit_enum_empty_enum() {
    struct EmptyEnum;

    impl<'de> EnumAccess<'de> for EmptyEnum {
        type Error = serde::de::Error;
        type Variant = EmptyVariant;

        fn variant<T>(self) -> Result<(T, Self::Variant), Self::Error>
        where
            T: Deserialize<'de>,
        {
            Err(serde::de::Error::custom("no variants"))
        }
    }

    struct EmptyVariant;

    let visitor = IgnoredAny;
    let enum_access = EmptyEnum;

    let _ = visitor.visit_enum(enum_access);
}

#[test]
fn test_visit_enum_multiple_variants() {
    struct MultipleVariantsEnum;

    enum MultiEnum {
        VariantA(IgnoredAny),
        VariantB(IgnoredAny),
        VariantC(IgnoredAny),
    }

    impl<'de> EnumAccess<'de> for MultipleVariantsEnum {
        type Error = serde::de::Error;
        type Variant = MultiVariant;

        fn variant<T>(self) -> Result<(T, Self::Variant), Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok((IgnoredAny, MultiVariant))
        }
    }

    struct MultiVariant;

    impl VariantAccess<'de> for MultiVariant {
        type Error = serde::de::Error;

        fn newtype_variant<T>(self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(IgnoredAny)
        }
    }

    let visitor = IgnoredAny;
    let enum_access = MultipleVariantsEnum;

    let _ = visitor.visit_enum(enum_access);
}

