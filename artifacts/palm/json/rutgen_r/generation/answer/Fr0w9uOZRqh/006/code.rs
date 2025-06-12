// Answer 0

#[test]
fn test_deserialize_any_null() {
    use serde_json::Value;
    use serde::de::Visitor;
    use serde::de::Error;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value, Error> {
            panic!("Should not receive a bool");
        }

        fn visit_string(self, _: String) -> Result<Self::Value, Error> {
            panic!("Should not receive a string");
        }

        fn visit_array(self, _: Vec<Self::Value>) -> Result<Self::Value, Error> {
            panic!("Should not receive an array");
        }

        fn visit_object<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            panic!("Should not receive an object");
        }

        fn visit_number(self, _: serde_json::Number) -> Result<Self::Value, Error> {
            panic!("Should not receive a number");
        }
    }

    let value = Value::Null;
    let visitor = TestVisitor;
    let result = value.deserialize_any(visitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_bool() {
    use serde_json::Value;
    use serde::de::Visitor;
    use serde::de::Error;

    struct TestVisitor {
        received: bool,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            panic!("Should not receive a unit");
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value, Error> {
            self.received = true;
            Ok(())
        }

        fn visit_string(self, _: String) -> Result<Self::Value, Error> {
            panic!("Should not receive a string");
        }

        fn visit_array(self, _: Vec<Self::Value>) -> Result<Self::Value, Error> {
            panic!("Should not receive an array");
        }

        fn visit_object<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            panic!("Should not receive an object");
        }

        fn visit_number(self, _: serde_json::Number) -> Result<Self::Value, Error> {
            panic!("Should not receive a number");
        }
    }

    let value = Value::Bool(true);
    let mut visitor = TestVisitor { received: false };
    let result = value.deserialize_any(visitor);
    assert!(result.is_ok());
    assert!(visitor.received);
}

#[test]
fn test_deserialize_any_number() {
    use serde_json::Value;
    use serde::de::Visitor;
    use serde::de::Error;

    struct TestVisitor {
        received: bool,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            panic!("Should not receive a unit");
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value, Error> {
            panic!("Should not receive a bool");
        }

        fn visit_string(self, _: String) -> Result<Self::Value, Error> {
            panic!("Should not receive a string");
        }

        fn visit_array(self, _: Vec<Self::Value>) -> Result<Self::Value, Error> {
            panic!("Should not receive an array");
        }

        fn visit_object<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            panic!("Should not receive an object");
        }

        fn visit_number(self, _: serde_json::Number) -> Result<Self::Value, Error> {
            self.received = true;
            Ok(())
        }
    }

    let value = Value::Number(serde_json::Number::from(42));
    let mut visitor = TestVisitor { received: false };
    let result = value.deserialize_any(visitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_array() {
    use serde_json::Value;
    use serde::de::Visitor;
    use serde::de::Error;

    struct TestVisitor {
        received: bool,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            panic!("Should not receive a unit");
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value, Error> {
            panic!("Should not receive a bool");
        }

        fn visit_string(self, _: String) -> Result<Self::Value, Error> {
            panic!("Should not receive a string");
        }

        fn visit_array(self, _: Vec<Self::Value>) -> Result<Self::Value, Error> {
            self.received = true;
            Ok(())
        }

        fn visit_object<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            panic!("Should not receive an object");
        }

        fn visit_number(self, _: serde_json::Number) -> Result<Self::Value, Error> {
            panic!("Should not receive a number");
        }
    }

    let value = Value::Array(vec![Value::Number(serde_json::Number::from(1)), Value::Bool(true)]);
    let mut visitor = TestVisitor { received: false };
    let result = value.deserialize_any(visitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_object() {
    use serde_json::Value;
    use serde::de::Visitor;
    use serde::de::Error;
    use serde::de::MapAccess;

    struct TestVisitor {
        received: bool,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            panic!("Should not receive a unit");
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value, Error> {
            panic!("Should not receive a bool");
        }

        fn visit_string(self, _: String) -> Result<Self::Value, Error> {
            panic!("Should not receive a string");
        }

        fn visit_array(self, _: Vec<Self::Value>) -> Result<Self::Value, Error> {
            panic!("Should not receive an array");
        }

        fn visit_object<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: MapAccess<'de>,
        {
            self.received = true;
            Ok(())
        }

        fn visit_number(self, _: serde_json::Number) -> Result<Self::Value, Error> {
            panic!("Should not receive a number");
        }
    }

    let value = Value::Object(serde_json::Map::new());
    let mut visitor = TestVisitor { received: false };
    let result = value.deserialize_any(visitor);
    assert!(result.is_ok());
}

