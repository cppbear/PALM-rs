// Answer 0

#[test]
fn test_iter_mut2() {
    struct MockBucket<K, V> {
        hash: u64,
        key: K,
        value: V,
    }

    struct MockEntries<K, V> {
        buckets: Vec<MockBucket<K, V>>,
    }

    impl<K, V> MockEntries<K, V> {
        fn as_entries_mut(&mut self) -> &mut [MockBucket<K, V>] {
            &mut self.buckets
        }
    }

    struct MockIndexMap<K, V> {
        entries: MockEntries<K, V>,
    }

    impl<K: Copy, V: Copy> MutableKeys for MockIndexMap<K, V> {
        type Key = K;
        type Value = V;

        fn get_full_mut2<Q>(&mut self, _key: &Q) -> Option<(usize, &mut K, &mut V)>
        where
            Q: ?Sized + Hash + Equivalent<K> {
            None // Not necessary for this test
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut K, &mut V)> {
            self.entries.buckets.get_mut(index).map(|bucket| (&mut bucket.key, &mut bucket.value))
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            IterMut2::new(self.entries.as_entries_mut())
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut K, &mut V) -> bool {
            // Not necessary for this test
        }
    }

    let mut index_map = MockIndexMap {
        entries: MockEntries {
            buckets: vec![
                MockBucket { hash: 1, key: 10, value: 'a' },
                MockBucket { hash: 2, key: 20, value: 'b' },
            ],
        },
    };

    let mut iter = index_map.iter_mut2();
    let first_bucket = iter.iter.next().unwrap();
    assert_eq!(first_bucket.key, 10);
    assert_eq!(first_bucket.value, 'a');

    let second_bucket = iter.iter.next().unwrap();
    assert_eq!(second_bucket.key, 20);
    assert_eq!(second_bucket.value, 'b');
}

#[test]
fn test_iter_mut2_empty() {
    struct MockBucket<K, V> {
        hash: u64,
        key: K,
        value: V,
    }

    struct MockEntries<K, V> {
        buckets: Vec<MockBucket<K, V>>,
    }

    impl<K, V> MockEntries<K, V> {
        fn as_entries_mut(&mut self) -> &mut [MockBucket<K, V>] {
            &mut self.buckets
        }
    }

    struct MockIndexMap<K, V> {
        entries: MockEntries<K, V>,
    }

    impl<K: Copy, V: Copy> MutableKeys for MockIndexMap<K, V> {
        type Key = K;
        type Value = V;

        fn get_full_mut2<Q>(&mut self, _key: &Q) -> Option<(usize, &mut K, &mut V)>
        where
            Q: ?Sized + Hash + Equivalent<K> {
            None // Not necessary for this test
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut K, &mut V)> {
            self.entries.buckets.get_mut(index).map(|bucket| (&mut bucket.key, &mut bucket.value))
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            IterMut2::new(self.entries.as_entries_mut())
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut K, &mut V) -> bool {
            // Not necessary for this test
        }
    }

    let mut index_map = MockIndexMap {
        entries: MockEntries { buckets: vec![] },
    };

    let mut iter = index_map.iter_mut2();
    assert!(iter.iter.next().is_none());
}

