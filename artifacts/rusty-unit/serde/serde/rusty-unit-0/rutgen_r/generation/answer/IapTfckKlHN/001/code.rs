// Answer 0

#[test]
fn test_deserialize_seq_invalid_type() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array or sequence")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    struct TestContent {
        content: Content,
    }

    impl TestContent {
        fn invalid_type<V>(&self, _visitor: &V) -> Result<V::Value, String> {
            Err("Invalid type".to_string())
        }

        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, String>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.content {
                Content::Seq(ref v) => visit_content_seq_ref(v, visitor),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    enum Content {
        Seq(Vec<i32>),
        Other(i32),
    }

    fn visit_content_seq_ref<V>(_v: &Vec<i32>, _visitor: V) -> Result<V::Value, String>
    where
        V: serde::de::Visitor<'de>,
    {
        Ok(_visitor.visit_unit().unwrap())
    }

    let invalid_content = TestContent {
        content: Content::Other(42),
    };

    let result: Result<(), String> = invalid_content.deserialize_seq(TestVisitor);

    assert_eq!(result, Err("Invalid type".to_string()));
}

