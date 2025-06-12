// Answer 0

#[test]
fn test_buckets() {
    struct TestTable {
        bucket_mask: usize,
    }

    impl TestTable {
        pub fn new(bucket_mask: usize) -> Self {
            TestTable { bucket_mask }
        }

        pub fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }
    }

    let table = TestTable::new(7);
    assert_eq!(table.buckets(), 8);

    let table_zero_mask = TestTable::new(0);
    assert_eq!(table_zero_mask.buckets(), 1);

    let table_large_mask = TestTable::new(1023);
    assert_eq!(table_large_mask.buckets(), 1024);
}

