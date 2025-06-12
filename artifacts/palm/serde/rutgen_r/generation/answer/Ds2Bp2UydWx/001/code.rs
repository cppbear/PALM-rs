// Answer 0

#[test]
fn test_deserialize_ignored_any() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, crate::de::Error> {
            Ok(())
        }
    }

    let visitor = TestVisitor;
    let result: Result<(), crate::de::Error> = deserialize_ignored_any(visitor);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_deserialize_ignored_any_with_panic() {
    struct PanicVisitor;

    impl<'de> Visitor<'de> for PanicVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, crate::de::Error> {
            panic!("This should trigger a panic");
        }
    }

    let visitor = PanicVisitor;
    let _ = deserialize_ignored_any(visitor);
}

