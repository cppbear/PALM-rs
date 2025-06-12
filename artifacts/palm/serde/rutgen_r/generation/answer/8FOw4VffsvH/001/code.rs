// Answer 0

#[test]
fn test_map_deserializer_new_with_iter() {
    struct DummyIter {
        data: Vec<(String, String)>,
        index: usize,
    }

    impl Iterator for DummyIter {
        type Item = (String, String);

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index].clone();
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let dummy_data = vec![
        (String::from("key1"), String::from("value1")),
        (String::from("key2"), String::from("value2")),
    ];
    
    let dummy_iter = DummyIter { data: dummy_data, index: 0 };
    let deserializer = serde::de::value::MapDeserializer::new(dummy_iter);

    assert!(deserializer.value.is_none());
    assert_eq!(deserializer.count, 0);
}

