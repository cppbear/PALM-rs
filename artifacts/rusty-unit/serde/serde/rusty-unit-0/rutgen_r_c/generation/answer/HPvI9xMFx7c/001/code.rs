// Answer 0

#[test]
fn test_visit_enum_err() {
    use crate::de::{EnumAccess, Error};
    
    struct TestEnumAccess;
    
    impl<'de> EnumAccess<'de> for TestEnumAccess {
        type Error = TestError;
        type Variant = TestVariantAccess;
        
        fn variant<T>(self) -> Result<(T, Self::Variant), Self::Error>
        where
            T: Deserialize<'de>,
        {
            Err(TestError)
        }
    }
    
    struct TestVariantAccess;
    
    impl VariantAccess<'de> for TestVariantAccess {
        fn newtype_variant<D>(self) -> Result<IgnoredAny, D::Error>
        where
            D: Deserializer<'de>,
        {
            Err(TestError)
        }
    }
    
    #[derive(Debug)]
    struct TestError;

    impl Error for TestError {
        fn custom<T>(_: T) -> Self where T: std::fmt::Debug {
            TestError
        }
    }

    let enum_access = TestEnumAccess;
    let result: Result<IgnoredAny, TestError> = enum_access.visit_enum();
    
    assert!(result.is_err());
}

