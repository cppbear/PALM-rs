// Answer 0

#[test]
fn test_deserialize_struct_with_map_content() {
    use crate::de::{Visitor, Error};

    struct TestVisitor {
        visited: bool,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            panic!("Should not visit seq");
        }
        
        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            self.visited = true;
            Ok(())
        }

        fn visit_none(self) -> Result<Self::Value, Self::Error> {
            self.visited = true;
            Ok(())
        }
        
        // Implement other required methods for the Visitor trait here as needed
    }

    let content = Content::Map(vec![
        (Content::String("key1".to_owned()), Content::String("value1".to_owned())),
        (Content::String("key2".to_owned()), Content::String("value2".to_owned())),
    ]);

    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData::<()>,
    };

    let visitor = TestVisitor { visited: false };

    let result = deserializer.deserialize_struct("TestStruct", &["key1", "key2"], visitor);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Should not visit seq")]
fn test_deserialize_struct_with_seq_content() {
    use crate::de::{Visitor, Error};

    struct TestVisitor {
        visited: bool,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            panic!("Should not visit map");
        }

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(())
        }
        
        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            self.visited = true;
            Ok(())
        }

        fn visit_none(self) -> Result<Self::Value, Self::Error> {
            self.visited = true;
            Ok(())
        }

        // Implement other required methods for the Visitor trait here as needed
    }

    let content = Content::Seq(vec![
        Content::String("item1".to_owned()),
        Content::String("item2".to_owned()),
    ]);

    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData::<()>,
    };

    let visitor = TestVisitor { visited: false };

    let _result = deserializer.deserialize_struct("TestStruct", &["item1", "item2"], visitor);
}

