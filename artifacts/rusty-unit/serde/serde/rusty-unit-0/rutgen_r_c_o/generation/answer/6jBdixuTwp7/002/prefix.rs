// Answer 0

#[test]
fn test_visit_enum_success() {
    struct TestEnum;

    impl EnumAccess<'static> for TestEnum {
        type Error = ();
        type Variant = TestVariantAccess;

        fn variant(self) -> Result<(Self::Variant, Self::VariantAccess), Self::Error> {
            Ok((TestVariantAccess, TestVariantAccess))
        }
    }
    
    struct TestVariantAccess;

    impl VariantAccess<'static> for TestVariantAccess {
        type Error = ();
        
        fn unit_variant(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let visitor = AdjacentlyTaggedEnumVariantVisitor {
        enum_name: "TestEnum",
        fields_enum: PhantomData,
    };

    let _ = visitor.visit_enum(TestEnum);
}

#[test]
fn test_visit_enum_variant_access_error() {
    struct TestEnum;

    impl EnumAccess<'static> for TestEnum {
        type Error = ();
        type Variant = TestVariantAccess;

        fn variant(self) -> Result<(Self::Variant, Self::VariantAccess), Self::Error> {
            Ok((TestVariantAccess, TestVariantAccess))
        }
    }

    struct TestVariantAccess;

    impl VariantAccess<'static> for TestVariantAccess {
        type Error = ();
        
        fn unit_variant(self) -> Result<(), Self::Error> {
            Err(())
        }
    }

    let visitor = AdjacentlyTaggedEnumVariantVisitor {
        enum_name: "TestEnum",
        fields_enum: PhantomData,
    };

    let _ = visitor.visit_enum(TestEnum);
}

#[test]
#[should_panic]
fn test_visit_enum_variant_error() {
    struct TestEnum;

    impl EnumAccess<'static> for TestEnum {
        type Error = ();
        type Variant = TestVariantAccess;

        fn variant(self) -> Result<(Self::Variant, Self::VariantAccess), Self::Error> {
            Err(())
        }
    }
    
    struct TestVariantAccess;

    impl VariantAccess<'static> for TestVariantAccess {
        type Error = ();
        
        fn unit_variant(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let visitor = AdjacentlyTaggedEnumVariantVisitor {
        enum_name: "TestEnum",
        fields_enum: PhantomData,
    };

    let _ = visitor.visit_enum(TestEnum);
}

