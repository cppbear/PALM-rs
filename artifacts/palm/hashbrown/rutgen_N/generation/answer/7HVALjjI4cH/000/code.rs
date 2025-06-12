// Answer 0

#[test]
fn test_replace_bucket_with_some() {
    struct TestTable<T> {
        ctrl: Vec<u8>,
        growth_left: usize,
        items: usize,
        buckets: Vec<Option<T>>,
    }

    struct TestHashMap<T> {
        table: TestTable<T>,
    }

    impl<T> TestHashMap<T> {
        fn bucket_index(&self, index: usize) -> usize {
            index
        }

        unsafe fn remove(&mut self, bucket: usize) -> (T, usize) {
            let item = self.table.buckets[bucket].take().unwrap();
            (item, bucket)
        }

        fn is_bucket_full(&self, index: usize) -> bool {
            self.table.buckets[index].is_some()
        }

        fn bucket(&mut self, index: usize) -> &mut Option<T> {
            &mut self.table.buckets[index]
        }

        fn set_ctrl(&mut self, index: usize, value: u8) {
            self.table.ctrl[index] = value;
        }
        
        fn new(size: usize) -> Self {
            TestHashMap {
                table: TestTable {
                    ctrl: vec![1; size],
                    growth_left: size,
                    items: 0,
                    buckets: vec![None; size],
                }
            }
        }
    }

    let mut hashmap = TestHashMap::new(5);
    hashmap.table.buckets[0] = Some(10);
    hashmap.table.items = 1; 

    unsafe {
        let result = hashmap.replace_bucket_with(0, |value| {
            Some(value + 1)
        });

        assert!(result);
        assert_eq!(hashmap.table.buckets[0], Some(11));
        assert_eq!(hashmap.table.items, 1);
    }
}

#[test]
fn test_replace_bucket_with_none() {
    struct TestTable<T> {
        ctrl: Vec<u8>,
        growth_left: usize,
        items: usize,
        buckets: Vec<Option<T>>,
    }

    struct TestHashMap<T> {
        table: TestTable<T>,
    }

    impl<T> TestHashMap<T> {
        fn bucket_index(&self, index: usize) -> usize {
            index
        }

        unsafe fn remove(&mut self, bucket: usize) -> (T, usize) {
            let item = self.table.buckets[bucket].take().unwrap();
            (item, bucket)
        }

        fn is_bucket_full(&self, index: usize) -> bool {
            self.table.buckets[index].is_some()
        }

        fn bucket(&mut self, index: usize) -> &mut Option<T> {
            &mut self.table.buckets[index]
        }

        fn set_ctrl(&mut self, index: usize, value: u8) {
            self.table.ctrl[index] = value;
        }

        fn new(size: usize) -> Self {
            TestHashMap {
                table: TestTable {
                    ctrl: vec![1; size],
                    growth_left: size,
                    items: 0,
                    buckets: vec![None; size],
                }
            }
        }
    }

    let mut hashmap = TestHashMap::new(5);
    hashmap.table.buckets[0] = Some(10);
    hashmap.table.items = 1; 

    unsafe {
        let result = hashmap.replace_bucket_with(0, |value| {
            None
        });

        assert!(!result);
        assert_eq!(hashmap.table.buckets[0], None);
        assert_eq!(hashmap.table.items, 0);
    }
}

