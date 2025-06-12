// Answer 0

#[test]
fn test_remove_all_extra_values() {
    #[derive(Debug, Clone)]
    enum Link {
        Extra(usize),
        None,
    }

    #[derive(Debug, Clone)]
    struct ExtraValue {
        value: usize,
        next: Link,
    }

    #[derive(Debug)]
    struct HeaderMap {
        extra_values: Vec<ExtraValue>,
    }

    impl HeaderMap {
        fn new() -> Self {
            HeaderMap {
                extra_values: Vec::new(),
            }
        }

        fn add_extra_value(&mut self, value: usize, next: Link) {
            self.extra_values.push(ExtraValue { value, next });
        }

        fn remove_extra_value(&mut self, idx: usize) -> ExtraValue {
            self.extra_values.remove(idx)
        }

        fn remove_all_extra_values(&mut self, mut head: usize) {
            loop {
                let extra = self.remove_extra_value(head);
                if let Link::Extra(idx) = extra.next {
                    head = idx;
                } else {
                    break;
                }
            }
        }
    }

    let mut header_map = HeaderMap::new();
    header_map.add_extra_value(1, Link::Extra(1));
    header_map.add_extra_value(2, Link::Extra(2));
    header_map.add_extra_value(3, Link::None); // This will stop the loop

    // This should remove all extra values linked in the structure
    header_map.remove_all_extra_values(0);

    // After the removal, we should only have one extra value (the last one linked to None)
    assert_eq!(header_map.extra_values.len(), 1);
    assert_eq!(header_map.extra_values[0].value, 3);
}

#[test]
#[should_panic]
fn test_remove_all_extra_values_invalid_index() {
    #[derive(Debug, Clone)]
    enum Link {
        Extra(usize),
        None,
    }

    #[derive(Debug, Clone)]
    struct ExtraValue {
        value: usize,
        next: Link,
    }

    #[derive(Debug)]
    struct HeaderMap {
        extra_values: Vec<ExtraValue>,
    }

    impl HeaderMap {
        fn new() -> Self {
            HeaderMap {
                extra_values: Vec::new(),
            }
        }

        fn add_extra_value(&mut self, value: usize, next: Link) {
            self.extra_values.push(ExtraValue { value, next });
        }

        fn remove_extra_value(&mut self, idx: usize) -> ExtraValue {
            self.extra_values.remove(idx)
        }

        fn remove_all_extra_values(&mut self, mut head: usize) {
            loop {
                let extra = self.remove_extra_value(head);
                if let Link::Extra(idx) = extra.next {
                    head = idx;
                } else {
                    break;
                }
            }
        }
    }

    let mut header_map = HeaderMap::new();
    header_map.add_extra_value(1, Link::Extra(1));
    header_map.add_extra_value(2, Link::Extra(2));
    header_map.add_extra_value(3, Link::Extra(3));
    // This is out of bound and will cause a panic
    header_map.remove_all_extra_values(10);
}

