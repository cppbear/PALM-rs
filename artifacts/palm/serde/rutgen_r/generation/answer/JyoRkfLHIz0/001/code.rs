// Answer 0

#[test]
fn test_deserialize_ignored_any_with_visitor() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not visit bool"))
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not visit i64"))
        }
    }

    let visitor = TestVisitor;
    let result: Result<(), serde::de::Error> = deserialize_ignored_any(visitor);
    
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "should not visit bool")]
fn test_deserialize_ignored_any_with_invalid_visit_bool() {
    struct InvalidVisitor;

    impl<'de> serde::de::Visitor<'de> for InvalidVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not call visit unit"))
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not visit bool"))
        }
    }

    let visitor = InvalidVisitor;
    let _result: Result<(), serde::de::Error> = deserialize_ignored_any(visitor);
}

