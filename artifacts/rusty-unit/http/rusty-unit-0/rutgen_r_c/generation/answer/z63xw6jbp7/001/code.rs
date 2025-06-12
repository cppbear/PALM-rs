// Answer 0

#[test]
fn test_insert_occupied_mult() {
    #[derive(Clone)]
    struct TestHeaderMap {
        entries: Vec<Bucket<HeaderValue>>,
        extra_values: Vec<ExtraValue<HeaderValue>>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                entries: vec![
                    Bucket {
                        hash: 0,
                        key: HeaderName::from_static("key1"),
                        value: HeaderValue::from_static("value1"),
                        links: None,
                    },
                    Bucket {
                        hash: 1,
                        key: HeaderName::from_static("key2"),
                        value: HeaderValue::from_static("value2"),
                        links: None,
                    },
                ],
                extra_values: vec![],
            }
        }

        fn raw_links(&mut self) -> RawLinks<HeaderValue> {
            RawLinks(&mut self.entries[..])
        }
    }

    let mut map = TestHeaderMap::new();
    let index = 0;
    let new_value = HeaderValue::from_static("new_value");

    let result = map.insert_occupied_mult(index, new_value.clone());

    assert_eq!(result.first, Some(HeaderValue::from_static("value1")));
    assert!(result.next.is_none());

    // Verify that the value at index 0 has been updated
    assert_eq!(map.entries[index].value, new_value);
}

