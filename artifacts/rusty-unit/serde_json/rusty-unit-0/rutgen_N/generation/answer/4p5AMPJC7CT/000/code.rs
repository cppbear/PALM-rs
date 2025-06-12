// Answer 0

#[test]
fn test_deserialize_struct_array() {
    use serde_json::Value;
    use serde::de::{self, Visitor};
    use std::marker::PhantomData;

    struct ArrayVisitor<T> {
        _phantom: PhantomData<T>,
    }

    impl<'de, T> Visitor<'de> for ArrayVisitor<T> {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of values")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element::<T>()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let value = Value::Array(vec![Value::Number(1.into()), Value::Number(2.into())]);
    let visitor = ArrayVisitor::<i32> { _phantom: PhantomData };
    let result: Result<Vec<i32>, _> = value.deserialize_struct("test", &[], visitor);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2]);
}

#[test]
fn test_deserialize_struct_object() {
    use serde_json::Value;
    use serde::de::{self, Visitor};
    use std::marker::PhantomData;

    #[derive(Debug, PartialEq)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    struct ObjectVisitor<'de> {
        _phantom: PhantomData<&'de ()>,
    }

    impl<'de> Visitor<'de> for ObjectVisitor<'de> {
        type Value = TestStruct;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct with two fields: field1 and field2")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            let mut field1 = None;
            let mut field2 = None;
            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "field1" => {
                        if field1.is_some() {
                            return Err(de::Error::duplicate_field("field1"));
                        }
                        field1 = Some(map.next_value()?);
                    }
                    "field2" => {
                        if field2.is_some() {
                            return Err(de::Error::duplicate_field("field2"));
                        }
                        field2 = Some(map.next_value()?);
                    }
                    _ => {
                        let _: de::IgnoredAny = map.next_value()?;
                    }
                }
            }
            let field1 = field1.ok_or_else(|| de::Error::missing_field("field1"))?;
            let field2 = field2.ok_or_else(|| de::Error::missing_field("field2"))?;
            Ok(TestStruct { field1, field2 })
        }
    }
    
    let value = Value::Object(
        serde_json::from_str(r#"{"field1":"value", "field2":42}"#).unwrap()
    );
    let visitor = ObjectVisitor { _phantom: PhantomData };
    let result: Result<TestStruct, _> = value.deserialize_struct("TestStruct", &["field1", "field2"], visitor);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), TestStruct { field1: "value".to_string(), field2: 42 });
}

#[test]
#[should_panic(expected = "expected a struct with two fields")]
fn test_deserialize_struct_invalid_type() {
    use serde_json::Value;
    use serde::de::{self, Visitor};
    use std::marker::PhantomData;

    struct InvalidVisitor<T> {
        _phantom: PhantomData<T>,
    }

    impl<'de, T> Visitor<'de> for InvalidVisitor<T> {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an invalid structure")
        }

        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
            Err(de::Error::custom("not a struct"))
        }
    }

    let value = Value::String("not a struct".to_string());
    let visitor = InvalidVisitor::<i32> { _phantom: PhantomData };
    let _result: Result<Vec<i32>, _> = value.deserialize_struct("test", &[], visitor).unwrap();
}

