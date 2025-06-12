// Answer 0

#[test]
fn test_visit_enum_success() {
    struct MockEnumAccess;
    
    impl<'de> EnumAccess<'de> for MockEnumAccess {
        type Error = ();
        
        fn variant(self) -> Result<(&'de str, MockVariantAccess), Self::Error> {
            Ok(("MockVariant", MockVariantAccess))
        }
    }
    
    struct MockVariantAccess;
    
    impl<'de> VariantAccess<'de> for MockVariantAccess {
        type Error = ();
        
        fn unit_variant(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }
    
    let visitor = AdjacentlyTaggedEnumVariantVisitor::<&str> {
        enum_name: "MockEnum",
        fields_enum: PhantomData,
    };
    
    let result = visitor.visit_enum(MockEnumAccess);
    assert_eq!(result, Ok("MockVariant"));
}

#[test]
fn test_visit_enum_variant_access_err() {
    struct MockEnumAccess;

    impl<'de> EnumAccess<'de> for MockEnumAccess {
        type Error = ();
        
        fn variant(self) -> Result<(&'de str, MockVariantAccessErr), Self::Error> {
            Ok(("MockVariant", MockVariantAccessErr))
        }
    }

    struct MockVariantAccessErr;

    impl<'de> VariantAccess<'de> for MockVariantAccessErr {
        type Error = ();

        fn unit_variant(self) -> Result<(), Self::Error> {
            Err(())
        }
    }

    let visitor = AdjacentlyTaggedEnumVariantVisitor::<&str> {
        enum_name: "MockEnum",
        fields_enum: PhantomData,
    };

    let result = visitor.visit_enum(MockEnumAccess);
    assert_eq!(result, Err(()));
}

