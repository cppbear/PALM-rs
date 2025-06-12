// Answer 0

#[test]
fn test_deserialize_unit_struct_success() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unit struct")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let result: Result<(), serde_json::Error> = serde_json::Deserializer::from_str("{}")
        .deserialize_unit_struct("TestStruct", TestVisitor);

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_deserialize_unit_struct_invalid_input() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unit struct")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            panic!("should not invoke visit_unit");
        }
    }

    let _result: Result<(), serde_json::Error> = serde_json::Deserializer::from_str("[1, 2, 3]")
        .deserialize_unit_struct("TestStruct", TestVisitor);
}

#[test]
fn test_deserialize_unit_struct_empty_object() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unit struct")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    let result: Result<(), serde_json::Error> = serde_json::Deserializer::from_str("{}")
        .deserialize_unit_struct("TestStruct", TestVisitor);

    assert!(result.is_ok());
}

