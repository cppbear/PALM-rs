// Answer 0

#[test]
fn test_new() {
    struct MockIndexMapCore<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K, V> MockIndexMapCore<K, V> {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn borrow_mut(&mut self) -> &mut Self {
            self
        }
    }

    struct Entry<'a, K, V> {
        map: &'a mut MockIndexMapCore<K, V>,
        index: usize,
    }

    let mut map = MockIndexMapCore::new();
    let index = 0;
    let entry = Entry::new(&mut map, index);

    assert_eq!(entry.index, index);
}

