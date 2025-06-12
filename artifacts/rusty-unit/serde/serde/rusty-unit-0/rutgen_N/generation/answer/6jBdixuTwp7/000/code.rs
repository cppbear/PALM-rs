// Answer 0

#[test]
fn test_visit_enum_unit_variant() {
    struct TestEnumAccess {
        called_variant: bool,
    }

    impl<'de> EnumAccess<'de> for TestEnumAccess {
        type Error = ();

        fn variant(self) -> Result<(usize, TestVariantAccess), Self::Error> {
            Ok((0, TestVariantAccess {}))
        }
    }

    struct TestVariantAccess;

    impl<'de> VariantAccess<'de> for TestVariantAccess {
        type Error = ();

        fn unit_variant(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let access = TestEnumAccess { called_variant: false };
    let result: Result<usize, ()> = visit_enum(access);
    assert_eq!(result, Ok(0));
}

#[test]
#[should_panic]
fn test_visit_enum_variant_access_fail() {
    struct FailingEnumAccess;

    impl<'de> EnumAccess<'de> for FailingEnumAccess {
        type Error = ();

        fn variant(self) -> Result<(usize, FailingVariantAccess), Self::Error> {
            Err(())
        }
    }

    struct FailingVariantAccess;

    impl<'de> VariantAccess<'de> for FailingVariantAccess {
        type Error = ();

        fn unit_variant(self) -> Result<(), Self::Error> {
            Err(())
        }
    }

    let access = FailingEnumAccess {};
    let _ = visit_enum(access);
}

