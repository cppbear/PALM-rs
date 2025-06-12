// Answer 0

#[test]
fn test_insert_entry_success() {
    struct Indices {
        data: Vec<usize>,
    }

    struct Entries<K, V> {
        data: Vec<Bucket<K, V>>,
    }

    #[derive(Debug)]
    struct Bucket<K, V> {
        hash: HashValue,
        key: K,
        value: V,
    }

    impl<K, V> Entries<K, V> {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn push(&mut self, bucket: Bucket<K, V>) {
            self.data.push(bucket);
        }

        fn capacity(&self) -> usize {
            self.data.capacity()
        }
    }

    impl Indices {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn insert_unique(&mut self, _hash: usize, index: usize, _hash_get: &usize) -> usize {
            self.data.push(index);
            index
        }
    }

    let mut indices = Indices::new();
    let mut entries: Entries<i32, &str> = Entries::new();

    let ref_mut = RefMut::new(&mut indices, &mut entries);
    let hash_value = HashValue(1);
    let key = 42;
    let value = "Test Value";

    let vacant_entry = VacantEntry {
        map: ref_mut,
        hash: hash_value,
        key,
    };

    let occupied_entry = vacant_entry.insert_entry(value);

    assert_eq!(entries.len(), 1);
    assert_eq!(entries.data[0].key, key);
    assert_eq!(entries.data[0].value, value);
}

#[test]
#[should_panic]
fn test_insert_entry_capacity_panic() {
    struct Indices {
        data: Vec<usize>,
    }

    struct Entries<K, V> {
        data: Vec<Bucket<K, V>>,
    }

    #[derive(Debug)]
    struct Bucket<K, V> {
        hash: HashValue,
        key: K,
        value: V,
    }

    impl<K, V> Entries<K, V> {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn push(&mut self, bucket: Bucket<K, V>) {
            self.data.push(bucket);
        }

        fn capacity(&self) -> usize {
            self.data.capacity()
        }
    }

    impl Indices {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn insert_unique(&mut self, _hash: usize, index: usize, _hash_get: &usize) -> usize {
            self.data.push(index);
            index
        }
    }

    let mut indices = Indices::new();
    let mut entries: Entries<i32, &str> = Entries::new();
    
    let ref_mut = RefMut::new(&mut indices, &mut entries);
    let hash_value = HashValue(2);
    let key = 43;
    let value = "Value That Exceeds Capacity";

    // Simulate maximum capacity without inserting any entries
    entries.data.reserve_exact(1);

    let vacant_entry = VacantEntry {
        map: ref_mut,
        hash: hash_value,
        key,
    };

    // This should panic as we are trying to insert while reaching a capacity limit
    vacant_entry.insert_entry(value);
}

