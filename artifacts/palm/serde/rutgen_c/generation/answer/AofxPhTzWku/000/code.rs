// Answer 0

#[test]
fn test_deserialize_seq_with_valid_visitor() {
    struct DummyVisitor {
        count: usize,
        visited: Vec<usize>,
    }

    impl<'de> de::Visitor<'de> for DummyVisitor {
        type Value = Vec<usize>;

        fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            while let Some(element) = visitor.next_element::<usize>()? {
                self.visited.push(element);
            }
            Ok(self.visited)
        }
    }

    let values = vec![1, 2, 3];
    let map_deserializer = MapDeserializer {
        iter: values.into_iter().fuse(),
        value: None,
        count: values.len(),
        lifetime: PhantomData,
        error: PhantomData,
    };

    let visitor = DummyVisitor { count: 3, visited: Vec::new() };
    let result = map_deserializer.deserialize_seq(visitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn test_deserialize_seq_with_empty_iterator() {
    struct DummyVisitor {
        count: usize,
    }

    impl<'de> de::Visitor<'de> for DummyVisitor {
        type Value = Vec<usize>;

        fn visit_seq<V>(self, visitor: V) -> Result<Self::Value, V::Error> 
        where
            V: de::SeqAccess<'de>,
        {
            let _ = visitor.next_element::<usize>()?;
            Ok(Vec::new())
        }
    }

    let empty: Vec<usize> = Vec::new();
    let map_deserializer = MapDeserializer {
        iter: empty.into_iter().fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    let visitor = DummyVisitor { count: 0 };
    let _ = map_deserializer.deserialize_seq(visitor);
}

#[test]
fn test_deserialize_seq_with_boundary_conditions() {
    struct DummyVisitor {
        expected_length: usize,
        visited_count: usize,
    }

    impl<'de> de::Visitor<'de> for DummyVisitor {
        type Value = usize;

        fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            while visitor.next_element::<usize>()?.is_some() {
                self.visited_count += 1;
            }
            Ok(self.visited_count)
        }
    }

    let values = vec![10, 20, 30];
    let map_deserializer = MapDeserializer {
        iter: values.into_iter().fuse(),
        value: None,
        count: values.len(),
        lifetime: PhantomData,
        error: PhantomData,
    };

    let visitor = DummyVisitor {
        expected_length: 3,
        visited_count: 0,
    };
    let result = map_deserializer.deserialize_seq(visitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3);
}

