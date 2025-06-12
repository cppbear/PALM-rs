// Answer 0

#[test]
fn test_get_index_valid_index() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    impl<K, V> Bucket<K, V> {
        fn refs(&self) -> (&K, &V) {
            (&self.key, &self.value)
        }
    }

    struct Slice<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> Slice<K, V> {
        fn new(entries: Vec<Bucket<K, V>>) -> Self {
            Self { entries }
        }

        pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
            self.entries.get(index).map(Bucket::refs)
        }
    }

    let bucket1 = Bucket { key: 1, value: "a" };
    let bucket2 = Bucket { key: 2, value: "b" };
    let slice = Slice::new(vec![bucket1, bucket2]);

    assert_eq!(slice.get_index(0), Some((&1, &"a")));
    assert_eq!(slice.get_index(1), Some((&2, &"b")));
}

#[test]
fn test_get_index_out_of_bounds() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    impl<K, V> Bucket<K, V> {
        fn refs(&self) -> (&K, &V) {
            (&self.key, &self.value)
        }
    }

    struct Slice<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> Slice<K, V> {
        fn new(entries: Vec<Bucket<K, V>>) -> Self {
            Self { entries }
        }

        pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
            self.entries.get(index).map(Bucket::refs)
        }
    }

    let slice = Slice::new(vec![
        Bucket { key: 1, value: "a" },
        Bucket { key: 2, value: "b" },
    ]);

    assert_eq!(slice.get_index(2), None);
    assert_eq!(slice.get_index(usize::MAX), None);
}

