// Answer 0

#[test]
fn test_erase_indices_non_empty() {
    #[derive(Debug)]
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl TestEntries {
        fn new() -> Self {
            TestEntries { entries: Vec::new() }
        }

        fn split_at(&mut self, index: usize) -> (Vec<Bucket<usize, usize>>, Vec<Bucket<usize, usize>>) {
            let (left, right) = self.entries.split_at(index);
            (left.to_vec(), right.to_vec())
        }

        fn push(&mut self, hash: HashValue, key: usize, value: usize) {
            self.entries.push(Bucket { hash, key, value });
        }
    }

    let mut map = IndexMapCore::new();
    map.entries = TestEntries::new();
    
    map.indices = Indices::with_capacity(10);
    
    let buckets = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
        Bucket { hash: HashValue(4), key: 4, value: 40 },
    ];
    
    for bucket in buckets {
        map.entries.push(bucket.hash, bucket.key, bucket.value);
    }
    
    map.indices.insert_unique(1, 0, |_| unreachable!());
    map.indices.insert_unique(2, 1, |_| unreachable!());
    map.indices.insert_unique(3, 2, |_| unreachable!());
    map.indices.insert_unique(4, 3, |_| unreachable!());
    
    let start = 1;
    let end = 3;

    // Executing the function to test
    map.erase_indices(start, end);

    // Validating the state after execution
    assert_eq!(map.indices.len(), 2);
    assert_eq!(map.indices.get(0), Some(&0));
    assert_eq!(map.indices.get(1), Some(&3));
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_erase_indices_panic_start_greater_than_end() {
    let mut map = IndexMapCore::new();
    
    let buckets = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];
    
    for bucket in buckets {
        map.entries.push(bucket.hash, bucket.key, bucket.value);
    }

    map.indices = Indices::with_capacity(2);
    map.indices.insert_unique(1, 0, |_| unreachable!());
    map.indices.insert_unique(2, 1, |_| unreachable!());

    // Panicking scenario with start greater than end
    map.erase_indices(2, 1);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_erase_indices_panic_split_at_empty() {
    let mut map = IndexMapCore::new();
    
    // Here, we're ensuring it's empty to cause a panic
    map.entries = TestEntries::new();
    map.indices = Indices::with_capacity(0);

    // This should panic since entries are empty
    map.erase_indices(0, 1);
}

