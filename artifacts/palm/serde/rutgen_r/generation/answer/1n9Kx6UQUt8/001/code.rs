// Answer 0

#[test]
fn test_deserialize_any_with_valid_visitor() {
    struct TestVisitor;
    
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                vec.push(value);
            }
            Ok(vec)
        }

        serde::de::forward_to_deserialize_any! {
            bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, char, str,
            bytes, unit, option, newtype_struct, tuple, tuple_struct, map,
            struct, identifier
        }
    }

    let data = serde_json::json!([1, 2, 3]);
    let result: Result<Vec<i32>, _> = serde_json::from_value(data).and_then(|value| {
        let mut de = serde_json::Deserializer::from_value(value);
        de.deserialize_any(TestVisitor)
    });

    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn test_deserialize_any_with_invalid_type() {
    struct PanicVisitor;

    impl<'de> serde::de::Visitor<'de> for PanicVisitor {
        type Value = ();

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            panic!("This visitor panics on purpose.");
        }

        serde::de::forward_to_deserialize_any! {
            bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, char, str,
            bytes, unit, option, newtype_struct, tuple, tuple_struct, map,
            struct, identifier
        }
    }

    let data = serde_json::json!([1, 2, 3]);
    let _result: Result<(), _> = serde_json::from_value(data).and_then(|value| {
        let mut de = serde_json::Deserializer::from_value(value);
        de.deserialize_any(PanicVisitor)
    });
}

