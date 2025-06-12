// Answer 0

#[test]
fn test_deserialize_any_string() {
    use serde::de::Visitor;
    use serde_json::{Value, Error};

    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(String::from("unit"))
        }

        fn visit_bool(self, _: bool) -> Result<Self::Value, Error> {
            Ok(String::from("bool"))
        }

        fn visit_string(self, value: String) -> Result<Self::Value, Error> {
            self.value = Some(value);
            Ok(self.value.clone().unwrap())
        }
        
        fn visit_array<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Ok(String::from("array"))
        }
        
        fn visit_map<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok(String::from("map"))
        }

        fn visit_f64(self, _: f64) -> Result<Self::Value, Error> {
            Ok(String::from("f64"))
        }

        fn visit_u64(self, _: u64) -> Result<Self::Value, Error> {
            Ok(String::from("u64"))
        }

        fn visit_i64(self, _: i64) -> Result<Self::Value, Error> {
            Ok(String::from("i64"))
        }

        fn visit_none(self) -> Result<Self::Value, Error> {
            Ok(String::from("none"))
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: serde::de::Deserialize<'de>,
        {
            Ok(String::from("some"))
        }
    }

    let value_string = Value::String(String::from("test string"));
    let visitor = TestVisitor { value: None };
    
    let result = value_string.deserialize_any(visitor).unwrap();
    assert_eq!(result, "test string");
}

