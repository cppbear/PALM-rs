// Answer 0

#[test]
fn test_deserialize_struct_with_seq() {
    struct TestVisitor {
        value: Option<Vec<i32>>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    struct TestDeserializer {
        content: Content,
    }

    impl TestDeserializer {
        fn invalid_type<V>(&self, _visitor: &V) -> &'static str {
            "invalid type"
        }

        fn deserialize_struct<V>(
            self,
            _name: &'static str,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, &'static str>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.content {
                Content::Seq(v) => visit_content_seq(v, visitor),
                Content::Map(v) => visit_content_map(v, visitor),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    #[derive(Debug)]
    enum Content {
        Seq(Vec<i32>),
        Map(std::collections::HashMap<String, i32>),
        Other,
    }

    fn visit_content_seq<V>(
        seq: Vec<i32>,
        visitor: V,
    ) -> Result<V::Value, &'static str>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut seq_access = SeqAccessWrapper(seq);
        visitor.visit_seq(seq_access)
    }

    fn visit_content_map<V>(
        _map: std::collections::HashMap<String, i32>,
        _visitor: V,
    ) -> Result<V::Value, &'static str>
    where
        V: serde::de::Visitor<'de>,
    {
        Err("not implemented for maps")
    }

    struct SeqAccessWrapper(Vec<i32>);

    impl<'de> serde::de::SeqAccess<'de> for SeqAccessWrapper {
        type Error = &'static str;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: serde::de::Deserialize<'de>,
        {
            if self.0.is_empty() {
                return Ok(None);
            }
            let value = self.0.remove(0);
            Ok(Some(value as T))
        }
    }

    let deserializer = TestDeserializer {
        content: Content::Seq(vec![1, 2, 3]),
    };

    let result: Result<Vec<i32>, _> = deserializer.deserialize_struct("Test", &[], TestVisitor { value: None });
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn test_deserialize_struct_invalid_type() {
    struct TestVisitor {
        value: Option<Vec<i32>>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<S>(self, _seq: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            panic!("should not reach here")
        }
    }

    struct TestDeserializer {
        content: Content,
    }

    impl TestDeserializer {
        fn invalid_type<V>(&self, _visitor: &V) -> &'static str {
            "invalid type"
        }
        
        fn deserialize_struct<V>(
            self,
            _name: &'static str,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, &'static str>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.content {
                Content::Seq(_) => {
                    // unable to provide valid sequence
                    Err(self.invalid_type(&visitor))
                },
                Content::Map(_) => {
                    // some handling for maps
                    Err(self.invalid_type(&visitor))
                },
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    #[derive(Debug)]
    enum Content {
        Seq(Vec<i32>),
        Map(std::collections::HashMap<String, i32>),
        Other,
    }

    let deserializer = TestDeserializer {
        content: Content::Other,
    };

    let _: Result<Vec<i32>, _> = deserializer.deserialize_struct("Test", &[], TestVisitor { value: None });
}

