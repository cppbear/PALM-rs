// Answer 0

#[test]
fn test_remove_all_extra_values_with_no_extra_values() {
    #[derive(Clone)]
    struct MockHeaderValue;

    #[derive(Debug)]
    struct MockHeaderMap {
        extra_values: Vec<ExtraValue<MockHeaderValue>>,
    }

    impl MockHeaderMap {
        fn new() -> Self {
            Self {
                extra_values: vec![],
            }
        }

        fn remove_extra_value(&mut self, _idx: usize) -> ExtraValue<MockHeaderValue> {
            panic!("No extra values to remove")
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

    let mut map = MockHeaderMap::new();
    let head = 0;
    map.remove_all_extra_values(head);
}

#[test]
fn test_remove_all_extra_values_with_single_extra_value() {
    #[derive(Clone)]
    struct MockHeaderValue;

    #[derive(Debug)]
    enum Link {
        Extra(usize),
        None,
    }

    #[derive(Debug)]
    struct ExtraValue<T> {
        value: T,
        next: Link,
    }

    #[derive(Debug)]
    struct MockHeaderMap {
        extra_values: Vec<ExtraValue<MockHeaderValue>>,
    }

    impl MockHeaderMap {
        fn new() -> Self {
            Self {
                extra_values: vec![ExtraValue {
                    value: MockHeaderValue,
                    next: Link::None,
                }],
            }
        }

        fn remove_extra_value(&mut self, idx: usize) -> ExtraValue<MockHeaderValue> {
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

    let mut map = MockHeaderMap::new();
    let head = 0;
    map.remove_all_extra_values(head);
}

#[test]
fn test_remove_all_extra_values_with_multiple_extra_values() {
    #[derive(Clone)]
    struct MockHeaderValue;

    #[derive(Debug)]
    enum Link {
        Extra(usize),
        None,
    }

    #[derive(Debug)]
    struct ExtraValue<T> {
        value: T,
        next: Link,
    }

    #[derive(Debug)]
    struct MockHeaderMap {
        extra_values: Vec<ExtraValue<MockHeaderValue>>,
    }

    impl MockHeaderMap {
        fn new() -> Self {
            Self {
                extra_values: vec![
                    ExtraValue {
                        value: MockHeaderValue,
                        next: Link::Extra(1),
                    },
                    ExtraValue {
                        value: MockHeaderValue,
                        next: Link::None,
                    },
                ],
            }
        }

        fn remove_extra_value(&mut self, idx: usize) -> ExtraValue<MockHeaderValue> {
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

    let mut map = MockHeaderMap::new();
    let head = 0;
    map.remove_all_extra_values(head);
}

