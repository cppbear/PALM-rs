// Answer 0

#[test]
fn test_insert() {
    struct TestBucket {
        value: Option<u64>,
    }

    struct TestTable {
        ctrl: Vec<u8>,
        buckets: Vec<TestBucket>,
        growth_left: usize,
    }

    impl TestTable {
        fn new(size: usize) -> Self {
            TestTable {
                ctrl: vec![0; size],
                buckets: vec![TestBucket { value: None }; size],
                growth_left: size,
            }
        }

        fn find_insert_slot(&mut self, hash: u64) -> InsertSlot {
            let index = (hash % self.buckets.len() as u64) as usize;
            InsertSlot { index }
        }
    }

    struct InsertSlot {
        index: usize,
    }

    impl TestTable {
        fn insert_in_slot(&mut self, hash: u64, slot: InsertSlot, value: u64) -> &mut TestBucket {
            self.buckets[slot.index].value = Some(value);
            &mut self.buckets[slot.index]
        }
    }

    let mut table = TestTable::new(4);
    let result = table.insert(1, 42, |x| *x);
    assert_eq!(result.value, Some(42));
}

#[test]
fn test_insert_overwrite() {
    struct TestBucket {
        value: Option<u64>,
    }

    struct TestTable {
        ctrl: Vec<u8>,
        buckets: Vec<TestBucket>,
        growth_left: usize,
    }

    impl TestTable {
        fn new(size: usize) -> Self {
            TestTable {
                ctrl: vec![0; size],
                buckets: vec![TestBucket { value: None }; size],
                growth_left: size,
            }
        }

        fn find_insert_slot(&mut self, hash: u64) -> InsertSlot {
            let index = (hash % self.buckets.len() as u64) as usize;
            InsertSlot { index }
        }
    }

    struct InsertSlot {
        index: usize,
    }

    impl TestTable {
        fn insert_in_slot(&mut self, hash: u64, slot: InsertSlot, value: u64) -> &mut TestBucket {
            self.buckets[slot.index].value = Some(value);
            &mut self.buckets[slot.index]
        }
    }

    let mut table = TestTable::new(4);
    table.insert(1, 42, |x| *x);
    let result = table.insert(1, 84, |x| *x);
    assert_eq!(result.value, Some(84));
}

