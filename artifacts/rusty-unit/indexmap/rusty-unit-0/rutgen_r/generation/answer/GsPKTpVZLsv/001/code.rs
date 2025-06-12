// Answer 0

#[test]
fn test_iter_mut_with_non_empty_map() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new(entries: Vec<(K, V)>) -> Self {
            TestMap { entries }
        }

        fn iter_mut(&mut self) -> std::slice::IterMut<'_, (K, V)> {
            self.entries.iter_mut()
        }
    }

    let mut map = TestMap::new(vec![(1, "a"), (2, "b"), (3, "c")]);
    let iter = map.iter_mut();

    let mut collected: Vec<_> = iter.collect();
    assert_eq!(collected, vec![&mut (1, "a"), &mut (2, "b"), &mut (3, "c")]);
}

#[test]
fn test_iter_mut_with_empty_map() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new(entries: Vec<(K, V)>) -> Self {
            TestMap { entries }
        }

        fn iter_mut(&mut self) -> std::slice::IterMut<'_, (K, V)> {
            self.entries.iter_mut()
        }
    }

    let mut map = TestMap::new(Vec::new());
    let iter = map.iter_mut();

    let collected: Vec<_> = iter.collect();
    assert!(collected.is_empty());
}

#[test]
#[should_panic]
fn test_iter_mut_panic_on_accessing_after_mutation() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new(entries: Vec<(K, V)>) -> Self {
            TestMap { entries }
        }

        fn iter_mut(&mut self) -> std::slice::IterMut<'_, (K, V)> {
            self.entries.iter_mut()
        }

        fn mutate_entries(&mut self) {
            if let Some(first) = self.entries.get_mut(0) {
                first.1 = "changed";
            }
        }
    }

    let mut map = TestMap::new(vec![(1, "a"), (2, "b")]);
    {
        let mut iter = map.iter_mut();
        let _ = iter.next(); // Consume the first item
        map.mutate_entries(); // Mutate while iterating
        // This should panic because we're accessing an entry after a mutation
        let _ = iter.next();
    }
}

