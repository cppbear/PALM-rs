// Answer 0

#[test]
fn test_find_none() {
    struct DummyTable<T> {
        buckets: Vec<Option<T>>,
    }

    impl<T> DummyTable<T> {
        fn new(size: usize) -> Self {
            Self {
                buckets: vec![None; size],
            }
        }

        fn buckets(&self) -> usize {
            self.buckets.len()
        }

        fn bucket(&self, index: usize) -> Option<&T> {
            self.buckets[index].as_ref()
        }
    }

    impl<T> DummyTable<T> {
        pub fn find(&self, hash: u64, mut eq: impl FnMut(&T) -> bool) -> Option<&T> {
            for (i, bucket) in self.buckets.iter().enumerate() {
                if bucket.is_some() && eq(bucket.as_ref().unwrap()) {
                    return Some(bucket.as_ref().unwrap());
                }
            }
            None
        }
    }

    let table: DummyTable<i32> = DummyTable::new(10);
    let result = table.find(12345, |&item| item == 42);
    assert!(result.is_none());
}

