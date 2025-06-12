// Answer 0

#[test]
fn test_unit_variant_success() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self { TestError }
    }
    
    let variant_access = UnitOnly::<TestError> { marker: PhantomData };
    let result = variant_access.unit_variant();
    assert!(result.is_ok());
}

#[test]
fn test_unit_variant_failure() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self { TestError }
    }

    struct TestVisitor;
    impl de::Visitor<'static> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Test visitor")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E> where E: de::Error {
            Ok(())
        }
    }
    
    let variant_access = UnitOnly::<TestError> { marker: PhantomData };
    let result = variant_access.newtype_variant_seed(TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_tuple_variant_failure() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self { TestError }
    }

    struct TestVisitor;
    impl de::Visitor<'static> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Test visitor")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E> where E: de::Error {
            Ok(())
        }
    }

    let variant_access = UnitOnly::<TestError> { marker: PhantomData };
    let result = variant_access.tuple_variant(1, TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_struct_variant_failure() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self { TestError }
    }

    struct TestVisitor;
    impl de::Visitor<'static> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Test visitor")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E> where E: de::Error {
            Ok(())
        }
    }
    
    let variant_access = UnitOnly::<TestError> { marker: PhantomData };
    let result = variant_access.struct_variant(&["field1", "field2"], TestVisitor);
    assert!(result.is_err());
}

