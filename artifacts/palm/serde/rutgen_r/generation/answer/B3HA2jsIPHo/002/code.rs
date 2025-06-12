// Answer 0

#[test]
fn test_struct_variant_with_map() {
    struct TestDeserializer {
        value: Option<Content>,
    }

    struct SimpleVisitor;

    impl<'de> de::Visitor<'de> for SimpleVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any map")
        }
        fn visit_map<M>(self, _: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    let map_content = Content::Map(std::collections::HashMap::<String, i32>::new());
    let deserializer = TestDeserializer {
        value: Some(map_content),
    };
    let result = deserializer.struct_variant(&[], SimpleVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_struct_variant_with_seq() {
    struct TestDeserializer {
        value: Option<Content>,
    }

    struct SimpleVisitor;

    impl<'de> de::Visitor<'de> for SimpleVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any sequence")
        }
        fn visit_seq<S>(self, _: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let seq_content = Content::Seq(vec![1, 2, 3]);
    let deserializer = TestDeserializer {
        value: Some(seq_content),
    };
    let result = deserializer.struct_variant(&[], SimpleVisitor);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "invalid type")]
fn test_struct_variant_with_unexpected_type() {
    struct TestDeserializer {
        value: Option<Content>,
    }

    struct SimpleVisitor;

    impl<'de> de::Visitor<'de> for SimpleVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }
    }

    let unexpected_content = Content::Bool(true);
    let deserializer = TestDeserializer {
        value: Some(unexpected_content),
    };
    let result = deserializer.struct_variant(&[], SimpleVisitor);
    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "invalid type")]
fn test_struct_variant_with_none() {
    struct TestDeserializer {
        value: Option<Content>,
    }

    struct SimpleVisitor;

    impl<'de> de::Visitor<'de> for SimpleVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct variant")
        }
    }

    let deserializer = TestDeserializer { value: None };
    let result = deserializer.struct_variant(&[], SimpleVisitor);
    assert!(result.is_err());
}

