// Answer 0

#[test]
fn test_iter_hash_valid_usage() {
    struct TestStruct {
        value: i32,
    }

    struct RawTable {
        occupied_buckets: Vec<(u64, TestStruct)>,
    }

    impl RawTable {
        fn new() -> Self {
            RawTable {
                occupied_buckets: Vec::new(),
            }
        }

        fn insert(&mut self, hash: u64, value: TestStruct) {
            self.occupied_buckets.push((hash, value));
        }
        
        unsafe fn iter_hash(&self, hash: u64) -> RawIterHash<TestStruct> {
            RawIterHash::new(self, hash)
        }
    }

    struct RawIterHash<'a, T> {
        table: &'a RawTable,
        hash: u64,
        index: usize,
    }

    impl<'a, T> RawIterHash<'a, T> {
        pub fn new(table: &'a RawTable, hash: u64) -> Self {
            RawIterHash { table, hash, index: 0 }
        }
        
        pub fn next(&mut self) -> Option<&'a T> {
            while self.index < self.table.occupied_buckets.len() {
                let (bucket_hash, value) = &self.table.occupied_buckets[self.index];
                self.index += 1;
                if *bucket_hash & 0x7F == hash & 0x7F { // Masking to 7 bits for comparison
                    return Some(value);
                }
            }
            None
        }
    }
    
    let mut table = RawTable::new();
    table.insert(1, TestStruct { value: 10 });
    table.insert(2, TestStruct { value: 20 });
    table.insert(129, TestStruct { value: 30 }); // 129 hashes to 1 in 7 bits

    unsafe {
        let mut iter = table.iter_hash(1);
        assert_eq!(iter.next().unwrap().value, 10);
        assert_eq!(iter.next().unwrap().value, 30);
        assert!(iter.next().is_none());
    }
}

#[test]
fn test_iter_hash_no_matches() {
    struct TestStruct {
        value: i32,
    }

    struct RawTable {
        occupied_buckets: Vec<(u64, TestStruct)>,
    }

    impl RawTable {
        fn new() -> Self {
            RawTable {
                occupied_buckets: Vec::new(),
            }
        }

        fn insert(&mut self, hash: u64, value: TestStruct) {
            self.occupied_buckets.push((hash, value));
        }
        
        unsafe fn iter_hash(&self, hash: u64) -> RawIterHash<TestStruct> {
            RawIterHash::new(self, hash)
        }
    }

    struct RawIterHash<'a, T> {
        table: &'a RawTable,
        hash: u64,
        index: usize,
    }

    impl<'a, T> RawIterHash<'a, T> {
        pub fn new(table: &'a RawTable, hash: u64) -> Self {
            RawIterHash { table, hash, index: 0 }
        }
        
        pub fn next(&mut self) -> Option<&'a T> {
            while self.index < self.table.occupied_buckets.len() {
                let (bucket_hash, value) = &self.table.occupied_buckets[self.index];
                self.index += 1;
                if *bucket_hash & 0x7F == hash & 0x7F { // Masking to 7 bits for comparison
                    return Some(value);
                }
            }
            None
        }
    }

    let mut table = RawTable::new();
    table.insert(3, TestStruct { value: 10 });
    table.insert(4, TestStruct { value: 20 });

    unsafe {
        let mut iter = table.iter_hash(1);
        assert!(iter.next().is_none());
    }
}

