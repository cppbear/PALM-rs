// Answer 0

#[test]
fn test_deserialize_any_true() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
        
        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            assert_eq!(value, true);
            Ok(())
        }

        fn visit_str(self, value: &str) -> Result<Self::Value> {
            assert_eq!(value, "test");
            Ok(())
        }

        fn visit_seq(self, _: SeqAccess<'de>) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_map(self, _: MapAccess<'de>) -> Result<Self::Value> {
            Ok(())
        }
    }

    let mut mock = MockDeserializer::new("true");
    let result = mock.deserialize_any(MockVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_false() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
        
        fn visit_bool(self, value: bool) -> Result<Self::Value> {
            assert_eq!(value, false);
            Ok(())
        }

        fn visit_str(self, value: &str) -> Result<Self::Value> {
            assert_eq!(value, "test");
            Ok(())
        }

        fn visit_seq(self, _: SeqAccess<'de>) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_map(self, _: MapAccess<'de>) -> Result<Self::Value> {
            Ok(())
        }
    }

    let mut mock = MockDeserializer::new("false");
    let result = mock.deserialize_any(MockVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_null() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value> {
            panic!("Expected unit, not bool");
        }

        fn visit_str(self, _: &str) -> Result<Self::Value> {
            panic!("Expected unit, not str");
        }

        fn visit_seq(self, _: SeqAccess<'de>) -> Result<Self::Value> {
            panic!("Expected unit, not seq");
        }

        fn visit_map(self, _: MapAccess<'de>) -> Result<Self::Value> {
            panic!("Expected unit, not map");
        }
    }

    let mut mock = MockDeserializer::new("null");
    let result = mock.deserialize_any(MockVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_number() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            panic!("Expected number, not unit");
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value> {
            panic!("Expected number, not bool");
        }

        fn visit_str(self, _: &str) -> Result<Self::Value> {
            panic!("Expected number, not str");
        }

        fn visit_seq(self, _: SeqAccess<'de>) -> Result<Self::Value> {
            panic!("Expected number, not seq");
        }

        fn visit_map(self, _: MapAccess<'de>) -> Result<Self::Value> {
            panic!("Expected number, not map");
        }
    }

    let mut mock = MockDeserializer::new("123");
    let result = mock.deserialize_any(MockVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_empty_array() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            panic!("Expected array, not unit");
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value> {
            panic!("Expected array, not bool");
        }

        fn visit_str(self, _: &str) -> Result<Self::Value> {
            panic!("Expected array, not str");
        }

        fn visit_seq(self, _: SeqAccess<'de>) -> Result<Self::Value> {
            Ok(())
        }

        fn visit_map(self, _: MapAccess<'de>) -> Result<Self::Value> {
            panic!("Expected array, not map");
        }
    }

    let mut mock = MockDeserializer::new("[]");
    let result = mock.deserialize_any(MockVisitor);
    assert!(result.is_ok());
}

