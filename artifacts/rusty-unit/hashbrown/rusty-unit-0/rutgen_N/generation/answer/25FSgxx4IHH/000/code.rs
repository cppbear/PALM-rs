// Answer 0

#[test]
fn test_erase_no_drop() {
    struct TestBucket {
        id: usize,
    }

    struct TestTable {
        items: Vec<Option<TestBucket>>,
    }

    impl TestTable {
        fn new(size: usize) -> Self {
            Self {
                items: vec![None; size],
            }
        }

        fn bucket_index(&self, bucket: &TestBucket) -> usize {
            bucket.id % self.items.len()
        }

        unsafe fn erase(&mut self, index: usize) {
            self.items[index] = None;
        }
    }

    struct TestHashBrown {
        table: TestTable,
    }

    impl TestHashBrown {
        fn new(size: usize) -> Self {
            Self {
                table: TestTable::new(size),
            }
        }

        unsafe fn erase_no_drop(&mut self, item: &TestBucket) {
            let index = self.table.bucket_index(item);
            self.table.erase(index);
        }
    }

    let mut hash_brown = TestHashBrown::new(10);
    let bucket = TestBucket { id: 5 };
    hash_brown.table.items[hash_brown.table.bucket_index(&bucket)] = Some(bucket);

    unsafe {
        hash_brown.erase_no_drop(&hash_brown.table.items[5].as_ref().unwrap());
    }

    assert!(hash_brown.table.items[hash_brown.table.bucket_index(&TestBucket { id: 5 })].is_none());
}

