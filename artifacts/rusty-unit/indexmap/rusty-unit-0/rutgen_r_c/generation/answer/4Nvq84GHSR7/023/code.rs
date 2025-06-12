// Answer 0

fn test_erase_indices_erased_zero() {
    #[derive(Debug)]
    struct TestEntries {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestEntries {
        fn new() -> Self {
            TestEntries { entries: Vec::new() }
        }

        fn add(&mut self, hash: usize, key: i32, value: i32) {
            self.entries.push(Bucket {
                hash: HashValue(hash),
                key,
                value,
            });
        }

        fn split_at(&mut self, index: usize) -> (Self, Self) {
            let (left, right) = self.entries.split_at(index);
            (
                TestEntries {
                    entries: left.to_vec(),
                },
                TestEntries {
                    entries: right.to_vec(),
                },
            )
        }
    }

    let mut core = IndexMapCore::new();
    core.entries = TestEntries::new().entries;

    // Condition: erased == 0
    core.erase_indices(0, 0);
}

fn test_erase_indices_start_plus_shifted_half_capacity() {
    #[derive(Debug)]
    struct TestEntries {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestEntries {
        fn new_with_capacity(capacity: usize) -> Self {
            TestEntries { entries: Vec::with_capacity(capacity) }
        }

        fn add(&mut self, hash: usize, key: i32, value: i32) {
            self.entries.push(Bucket {
                hash: HashValue(hash),
                key,
                value,
            });
        }
    }

    let mut core = IndexMapCore::with_capacity(4);
    core.entries = TestEntries::new_with_capacity(4).entries;

    // Fill entries to MAX capacity while having erased and shifted condition as described
    for i in 0..4 {
        core.entries.push(Bucket {
            hash: HashValue(i),
            key: i,
            value: i * 10,
        });
    }
    
    // Constraints: start + shifted == half_capacity = 4
    core.erase_indices(2, 4);
}

fn test_erase_indices_erased_plus_shifted_half_capacity() {
    #[derive(Debug)]
    struct TestEntries {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestEntries {
        fn new_with_capacity(capacity: usize) -> Self {
            TestEntries { entries: Vec::with_capacity(capacity) }
        }

        fn add(&mut self, hash: usize, key: i32, value: i32) {
            self.entries.push(Bucket {
                hash: HashValue(hash),
                key,
                value,
            });
        }
    }

    let mut core = IndexMapCore::with_capacity(6);
    core.entries = TestEntries::new_with_capacity(6).entries;

    // Fill entries to MAX capacity while having erased and shifted condition as described
    for i in 0..6 {
        core.entries.push(Bucket {
            hash: HashValue(i),
            key: i,
            value: i * 20,
        });
    }
    
    // Constraints: erased + shifted == half_capacity = 6
    core.erase_indices(3, 6);
}

fn test_erase_indices_left_equals_right() {
    #[derive(Debug)]
    struct TestEntries {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestEntries {
        fn new() -> Self {
            TestEntries { entries: Vec::new() }
        }

        fn add(&mut self, hash: usize, key: i32, value: i32) {
            self.entries.push(Bucket {
                hash: HashValue(hash),
                key,
                value,
            });
        }
    }

    let mut core = IndexMapCore::new();
    core.entries = TestEntries::new().entries;

    // Add test data that satisfies the left equals right condition
    core.entries.push(Bucket {
        hash: HashValue(1),
        key: 1,
        value: 100,
    });
    core.entries.push(Bucket {
        hash: HashValue(1),
        key: 1,
        value: 200,
    });

    // Ensure the values are equal - special case to trigger the specific check
    core.erase_indices(0, 2);
}

