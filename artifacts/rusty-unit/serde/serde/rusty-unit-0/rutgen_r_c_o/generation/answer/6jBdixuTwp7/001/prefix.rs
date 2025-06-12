// Answer 0

#[test]
#[should_panic]
fn test_visit_enum_invalid_variant() {
    struct InvalidEnum;

    impl<'de> EnumAccess<'de> for InvalidEnum {
        type Error = &'static str;
        type Variant = InvalidVariantAccess;

        fn variant(self) -> Result<(Self::Variant, Self::Error), Self::Error> {
            Err("Invalid variant access")
        }
    }

    struct InvalidVariantAccess;

    impl<'de> VariantAccess<'de> for InvalidVariantAccess {
        type Error = &'static str;

        fn unit_variant(self) -> Result<(), Self::Error> {
            Err("Invalid unit variant access")
        }
    }

    let visitor = AdjacentlyTaggedEnumVariantVisitor {
        enum_name: "InvalidEnum",
        fields_enum: PhantomData,
    };
    let _ = visitor.visit_enum(InvalidEnum);
}

