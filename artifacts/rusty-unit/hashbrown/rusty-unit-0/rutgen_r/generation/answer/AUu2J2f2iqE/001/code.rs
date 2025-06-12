// Answer 0

#[test]
fn test_iter_hash_with_occupied_buckets() {
    struct RawTable {
        buckets: Vec<Option<u64>>, // Simulating occupied buckets with hashes
    }

    struct RawIterHash<'a> {
        table: &'a RawTable,
        current: usize,
        hash: u64,
    }

    impl<'a> RawIterHash<'a> {
        fn new(table: &'a RawTable, hash: u64) -> Self {
            RawIterHash { table, current: 0, hash }
        }

        fn next(&mut self) -> Option<u64> {
            while self.current < self.table.buckets.len() {
                if let Some(&bucket_hash) = self.table.buckets.get(self.current) {
                    if (bucket_hash & 0x7F) == (self.hash & 0x7F) { // checking the last 7 bits
                        self.current += 1;
                        return Some(bucket_hash);
                    }
                }
                self.current += 1;
            }
            None
        }
    }

    let table = RawTable {
        buckets: vec![Some(0b0000000), Some(0b1111111), Some(0b0000001), None],
    };

    unsafe {
        let mut iter = table.iter_hash(0b0000000);
        assert_eq!(iter.next(), Some(0b0000000));
        assert_eq!(iter.next(), Some(0b1111111));
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn test_iter_hash_empty_buckets() {
    struct RawTable {
        buckets: Vec<Option<u64>>,
    }

    struct RawIterHash<'a> {
        table: &'a RawTable,
        current: usize,
        hash: u64,
    }

    impl<'a> RawIterHash<'a> {
        fn new(table: &'a RawTable, hash: u64) -> Self {
            RawIterHash { table, current: 0, hash }
        }

        fn next(&mut self) -> Option<u64> {
            while self.current < self.table.buckets.len() {
                if let Some(&bucket_hash) = self.table.buckets.get(self.current) {
                    if (bucket_hash & 0x7F) == (self.hash & 0x7F) {
                        self.current += 1;
                        return Some(bucket_hash);
                    }
                }
                self.current += 1;
            }
            None
        }
    }

    let table = RawTable { buckets: vec![] };

    unsafe {
        let mut iter = table.iter_hash(0b0000000);
        assert_eq!(iter.next(), None);
    }
}

#[should_panic]
#[test]
fn test_iter_hash_with_null_buckets() {
    struct RawTable {
        buckets: Vec<Option<u64>>,
    }

    struct RawIterHash<'a> {
        table: &'a RawTable,
        current: usize,
        hash: u64,
    }

    impl<'a> RawIterHash<'a> {
        fn new(table: &'a RawTable, hash: u64) -> Self {
            RawIterHash { table, current: 0, hash }
        }

        fn next(&mut self) -> Option<u64> {
            while self.current < self.table.buckets.len() {
                if let Some(&bucket_hash) = self.table.buckets.get(self.current) {
                    if (bucket_hash & 0x7F) == (self.hash & 0x7F) {
                        self.current += 1;
                        return Some(bucket_hash);
                    }
                }
                self.current += 1;
            }
            None
        }
    }

    // A case where accessing null (uninitialized) buckets might panic
    let table = RawTable { buckets: vec![None, None] };

    unsafe {
        let mut iter = table.iter_hash(0b0000000);
        iter.next(); // This should not panic on None value
        iter.next(); // This should also not panic
        iter.next(); // Should exit cleanly as well
    }
}

