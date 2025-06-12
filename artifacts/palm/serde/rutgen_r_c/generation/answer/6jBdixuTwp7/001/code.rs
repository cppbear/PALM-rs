// Answer 0

#[test]
fn test_visit_enum_returns_error_on_variant_failure() {
    struct MockEnumAccess<'de> {
        should_fail: bool,
        phantom: std::marker::PhantomData<&'de ()>,
    }

    impl<'de> EnumAccess<'de> for MockEnumAccess<'de> {
        type Error = &'static str;

        fn variant(self) -> Result<(u8, MockVariantAccess<'de>), Self::Error> {
            if self.should_fail {
                Err("Variant access failed")
            } else {
                Ok((0, MockVariantAccess { phantom: std::marker::PhantomData }))
            }
        }
    }

    struct MockVariantAccess<'de> {
        phantom: std::marker::PhantomData<&'de ()>,
    }

    impl<'de> VariantAccess<'de> for MockVariantAccess<'de> {
        type Error = &'static str;

        fn unit_variant(self) -> Result<(), Self::Error> {
            Err("Unit variant access failed")
        }
    }

    let visitor = AdjacentlyTaggedEnumVariantVisitor::<()> { enum_name: "MockEnum", fields_enum: std::marker::PhantomData };
    let mock_enum_access = MockEnumAccess { should_fail: true, phantom: std::marker::PhantomData };

    let result = visitor.visit_enum(mock_enum_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_enum_returns_variant_on_success() {
    struct MockEnumAccess<'de> {
        should_fail: bool,
        phantom: std::marker::PhantomData<&'de ()>,
    }

    impl<'de> EnumAccess<'de> for MockEnumAccess<'de> {
        type Error = &'static str;

        fn variant(self) -> Result<(u8, MockVariantAccess<'de>), Self::Error> {
            if self.should_fail {
                Err("Variant access failed")
            } else {
                Ok((0, MockVariantAccess { phantom: std::marker::PhantomData }))
            }
        }
    }

    struct MockVariantAccess<'de> {
        phantom: std::marker::PhantomData<&'de ()>,
    }

    impl<'de> VariantAccess<'de> for MockVariantAccess<'de> {
        type Error = &'static str;

        fn unit_variant(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let visitor = AdjacentlyTaggedEnumVariantVisitor::<()> { enum_name: "MockEnum", fields_enum: std::marker::PhantomData };
    let mock_enum_access = MockEnumAccess { should_fail: false, phantom: std::marker::PhantomData };

    let result = visitor.visit_enum(mock_enum_access);
    assert!(result.is_ok());
}

