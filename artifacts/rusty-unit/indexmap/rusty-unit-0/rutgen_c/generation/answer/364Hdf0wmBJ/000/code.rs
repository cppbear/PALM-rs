// Answer 0

#[test]
fn test_insert_unique() {
    struct TestEntries {
        data: Vec<Bucket<usize, usize>>,
    }

    impl TestEntries {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn capacity(&self) -> usize {
            self.data.capacity()
        }

        fn push(&mut self, bucket: Bucket<usize, usize>) {
            self.data.push(bucket);
        }

        fn reserve_exact(&mut self, additional: usize) {
            self.data.reserve_exact(additional);
        }

        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
            self.data.try_reserve_exact(additional).map_err(|_| TryReserveError)
        }
    }

    struct TestIndices {
        data: Vec<usize>,
    }

    impl TestIndices {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn insert_unique(&mut self, hash: u64, index: usize, _get_hash: impl Fn(&usize) -> u64) -> usize {
            self.data.push(index);
            index
        }
    }

    let mut indices = TestIndices::new();
    let mut entries = TestEntries::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    let hash_value = HashValue(42);
    let key = 1;
    let value = 100;
    
    let occupied_entry = ref_mut.insert_unique(hash_value, key, value);

    assert_eq!(occupied_entry.index(), 0);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries.data[0].key, key);
    assert_eq!(entries.data[0].value, value);
} 

#[test]
fn test_insert_unique_increases_capacity() {
    struct TestEntries {
        data: Vec<Bucket<usize, usize>>,
    }

    impl TestEntries {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn capacity(&self) -> usize {
            self.data.capacity()
        }

        fn push(&mut self, bucket: Bucket<usize, usize>) {
            self.data.push(bucket);
        }

        fn reserve_exact(&mut self, additional: usize) {
            self.data.reserve_exact(additional);
        }

        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
            self.data.try_reserve_exact(additional).map_err(|_| TryReserveError)
        }
    }

    struct TestIndices {
        data: Vec<usize>,
    }

    impl TestIndices {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn insert_unique(&mut self, hash: u64, index: usize, _get_hash: impl Fn(&usize) -> u64) -> usize {
            self.data.push(index);
            index
        }
    }

    let mut indices = TestIndices::new();
    let mut entries = TestEntries::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    for i in 0..10 {
        assert_eq!(entries.len(), i);
        ref_mut.insert_unique(HashValue(i as usize), i, i * 100);
        if i >= entries.capacity() {
            assert!(entries.capacity() >= i + 1);
        }
    }
}

