// Answer 0

#[test]
fn test_deserialize_option_null() {
    struct TestVisitor {
        visited_none: bool,
        visited_some: bool,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not visit some"))
        }
    }

    let value = Value::Null;
    let visitor = TestVisitor {
        visited_none: false,
        visited_some: false,
    };

    assert!(value.deserialize_option(visitor).is_ok());
}

#[test]
fn test_deserialize_option_some() {
    struct TestVisitor {
        visited_some: bool,
        visited_none: bool,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("should not visit none"))
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, serde::de::Error> {
            Ok(())
        }
    }

    let value = Value::Bool(true);
    let visitor = TestVisitor {
        visited_none: false,
        visited_some: false,
    };

    assert!(value.deserialize_option(visitor).is_ok());
}

