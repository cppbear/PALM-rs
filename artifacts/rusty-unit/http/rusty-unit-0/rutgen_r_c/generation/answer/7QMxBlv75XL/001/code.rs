// Answer 0

#[test]
fn test_remove_all_extra_values() {
    #[derive(Debug, Clone)]
    struct MockHeaderMap {
        extra_values: Vec<ExtraValue<HeaderValue>>,
    }

    impl MockHeaderMap {
        fn new() -> Self {
            MockHeaderMap {
                extra_values: Vec::new(),
            }
        }

        fn remove_extra_value(&mut self, idx: usize) -> ExtraValue<HeaderValue> {
            self.extra_values.remove(idx)
        }

        fn add_extra_value(&mut self, value: HeaderValue, next_index: Option<usize>) -> usize {
            let idx = self.extra_values.len();
            self.extra_values.push(ExtraValue {
                value,
                prev: Link::Null, // Assuming `Link::Null` as one of the options
                next: match next_index {
                    Some(i) => Link::Extra(i),
                    None => Link::Null,
                },
            });
            idx
        }
    }

    let mut map = MockHeaderMap::new();
    
    // Create mock extra values where the next links form a chain
    let v1 = HeaderValue::from("value1");
    let v2 = HeaderValue::from("value2");
    let v3 = HeaderValue::from("value3");
    
    // Setting up a chain: value1 -> value2 -> value3 -> None
    let idx1 = map.add_extra_value(v1, Some(1));
    let idx2 = map.add_extra_value(v2, Some(2));
    let idx3 = map.add_extra_value(v3, None);

    // Now we run the method under test
    map.remove_all_extra_values(idx1);

    // Assert the extra values are correctly removed: should only contain value3
    assert_eq!(map.extra_values.len(), 1);
    assert_eq!(map.extra_values[0].value, v3);
}

