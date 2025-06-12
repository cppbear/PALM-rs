// Answer 0

fn test_deserialize_enum_valid() {
    struct TestVisitor {
        result: Option<(String, Value)>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = (String, Value);

        fn visit_enum<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: VariantAccess<'de>,
        {
            Ok(self.result.unwrap())
        }
    }

    let test_map: Map<String, Value> = Map {
        map: MapImpl::new(vec![("variant_key".to_string(), Value::Number(Number::from(42)))].into_iter().collect()),
    };

    let visitor = TestVisitor {
        result: Some(("variant_key".to_string(), Value::Number(Number::from(42)))),
    };

    let result = test_map.deserialize_enum("TestEnum", &["variant_key"], visitor).unwrap();
    assert_eq!(result, ("variant_key".to_string(), Value::Number(Number::from(42))));
}

fn test_deserialize_enum_empty_map() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_enum<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: VariantAccess<'de>,
        {
            panic!("This should not be called.");
        }
    }

    let test_map: Map<String, Value> = Map {
        map: MapImpl::new(vec![].into_iter().collect()),
    };

    let visitor = TestVisitor;
    let result = test_map.deserialize_enum("TestEnum", &["variant_key"], visitor);
    assert!(result.is_err());
}

fn test_deserialize_enum_multiple_keys() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_enum<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: VariantAccess<'de>,
        {
            panic!("This should not be called.");
        }
    }

    let test_map: Map<String, Value> = Map {
        map: MapImpl::new(vec![
            ("key1".to_string(), Value::Number(Number::from(42))),
            ("key2".to_string(), Value::Number(Number::from(100))),
        ].into_iter().collect()),
    };

    let visitor = TestVisitor;
    let result = test_map.deserialize_enum("TestEnum", &["variant_key"], visitor);
    assert!(result.is_err());
}

