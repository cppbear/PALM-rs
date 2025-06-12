// Answer 0

#[test]
fn test_struct_variant_with_object() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    use std::marker::PhantomData;

    struct TestVisitor {
        _phantom: PhantomData<()>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, de::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(Value::Object(serde_json::Map::new()))
        }
    }

    struct TestStruct {
        value: Option<Value>,
    }

    impl TestStruct {
        fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, de::Error>
        where
            V: Visitor<'de>,
        {
            match self.value {
                Some(Value::Object(v)) => v.deserialize_any(visitor),
                Some(other) => Err(de::Error::invalid_type(
                    other.unexpected(),
                    &"struct variant",
                )),
                None => Err(de::Error::invalid_type(
                    de::Unexpected::UnitVariant,
                    &"struct variant",
                )),
            }
        }
    }

    let test_struct = TestStruct {
        value: Some(Value::Object(serde_json::Map::new())),
    };

    let result = test_struct.struct_variant(&["field1", "field2"], TestVisitor { _phantom: PhantomData });
    assert!(result.is_ok());
}

#[test]
fn test_struct_variant_with_non_object() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    use std::marker::PhantomData;

    struct TestVisitor {
        _phantom: PhantomData<()>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, de::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(Value::Object(serde_json::Map::new()))
        }
    }

    struct TestStruct {
        value: Option<Value>,
    }

    impl TestStruct {
        fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, de::Error>
        where
            V: Visitor<'de>,
        {
            match self.value {
                Some(Value::Object(v)) => v.deserialize_any(visitor),
                Some(other) => Err(de::Error::invalid_type(
                    other.unexpected(),
                    &"struct variant",
                )),
                None => Err(de::Error::invalid_type(
                    de::Unexpected::UnitVariant,
                    &"struct variant",
                )),
            }
        }
    }

    let test_struct = TestStruct {
        value: Some(Value::Bool(true)),
    };

    let result = test_struct.struct_variant(&["field1", "field2"], TestVisitor { _phantom: PhantomData });
    assert!(result.is_err());
}

#[test]
fn test_struct_variant_with_none() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    use std::marker::PhantomData;

    struct TestVisitor {
        _phantom: PhantomData<()>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, de::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(Value::Object(serde_json::Map::new()))
        }
    }

    struct TestStruct {
        value: Option<Value>,
    }

    impl TestStruct {
        fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, de::Error>
        where
            V: Visitor<'de>,
        {
            match self.value {
                Some(Value::Object(v)) => v.deserialize_any(visitor),
                Some(other) => Err(de::Error::invalid_type(
                    other.unexpected(),
                    &"struct variant",
                )),
                None => Err(de::Error::invalid_type(
                    de::Unexpected::UnitVariant,
                    &"struct variant",
                )),
            }
        }
    }

    let test_struct = TestStruct { value: None };
    let result = test_struct.struct_variant(&["field1", "field2"], TestVisitor { _phantom: PhantomData });
    assert!(result.is_err());
}

