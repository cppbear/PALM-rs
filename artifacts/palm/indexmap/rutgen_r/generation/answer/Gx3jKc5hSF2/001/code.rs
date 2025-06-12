// Answer 0

fn reverse_test() {
    struct TestEntries {
        entries: Vec<i32>,
        indices: Vec<usize>,
    }

    // Test case for normal operation
    {
        let mut test = TestEntries {
            entries: vec![1, 2, 3],
            indices: vec![0, 1, 2],
        };
        test.entries.reverse();
        let len = test.entries.len();
        for i in &mut test.indices {
            *i = len - *i - 1;
        }
        assert_eq!(test.entries, vec![3, 2, 1]);
        assert_eq!(test.indices, vec![2, 1, 0]);
    }

    // Test case for boundary condition: empty entries and indices
    {
        let mut test = TestEntries {
            entries: vec![],
            indices: vec![],
        };
        test.entries.reverse();
        let len = test.entries.len();
        for i in &mut test.indices {
            *i = len - *i - 1;
        }
        assert_eq!(test.entries, vec![]);
        assert_eq!(test.indices, vec![]);
    }

    // Test case for single element entries and indices
    {
        let mut test = TestEntries {
            entries: vec![42],
            indices: vec![0],
        };
        test.entries.reverse();
        let len = test.entries.len();
        for i in &mut test.indices {
            *i = len - *i - 1;
        }
        assert_eq!(test.entries, vec![42]);
        assert_eq!(test.indices, vec![0]);
    }

    // Test case with negative indices (presents invalid state but needed for testing)
    {
        let mut test = TestEntries {
            entries: vec![1, 2, 3],
            indices: vec![0, 1, 5],
        };
        // Simulating panic scenario for invalid index by catching panic
        let result = std::panic::catch_unwind(|| {
            test.entries.reverse();
            let len = test.entries.len();
            for i in &mut test.indices {
                *i = len - *i - 1; // This will lead to invalid index access
            }
        });
        assert!(result.is_err()); // Verify that panic occurred
    }
}

