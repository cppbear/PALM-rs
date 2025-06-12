// Answer 0

#[test]
fn test_visit_enum_empty() {
    struct EmptyEnumVisitor;

    impl<'de> EnumAccess<'de> for EmptyEnumVisitor {
        type Error = serde::de::serde::de::Error;
        type Variant = ();

        fn variant(self) -> Result<(String, Self::Variant), Self::Error> {
            Err(serde::de::serde::de::Error::custom("invalid variant"))
        }
    }

    let visitor = EmptyEnumVisitor;
    let content_visitor = ContentVisitor { value: PhantomData };
    let _ = content_visitor.visit_enum(visitor);
}

#[test]
fn test_visit_enum_other_variant() {
    struct OtherVariantVisitor;

    impl<'de> EnumAccess<'de> for OtherVariantVisitor {
        type Error = serde::de::serde::de::Error;
        type Variant = ();

        fn variant(self) -> Result<(String, Self::Variant), Self::Error> {
            Err(serde::de::serde::de::Error::custom("invalid variant"))
        }
    }

    let visitor = OtherVariantVisitor;
    let content_visitor = ContentVisitor { value: PhantomData };
    let _ = content_visitor.visit_enum(visitor);
}

#[test]
fn test_visit_enum_specific_variant() {
    struct SpecificVariantVisitor;

    impl<'de> EnumAccess<'de> for SpecificVariantVisitor {
        type Error = serde::de::serde::de::Error;
        type Variant = ();

        fn variant(self) -> Result<(String, Self::Variant), Self::Error> {
            Err(serde::de::serde::de::Error::custom("invalid variant"))
        }
    }

    let visitor = SpecificVariantVisitor;
    let content_visitor = ContentVisitor { value: PhantomData };
    let _ = content_visitor.visit_enum(visitor);
}

