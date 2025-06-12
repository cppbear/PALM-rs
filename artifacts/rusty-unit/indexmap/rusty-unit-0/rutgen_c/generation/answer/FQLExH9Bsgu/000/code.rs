// Answer 0

#[test]
fn test_as_mut_slice_empty() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, i32>] {
            &mut self.entries
        }

        fn as_mut_slice(&mut self) -> &mut Slice<i32, i32> {
            Slice::from_mut_slice(self.as_entries_mut())
        }
    }

    let mut map = TestMap { entries: Vec::new() };
    let slice = map.as_mut_slice();
    assert_eq!(slice.entries.len(), 0);
}

#[test]
fn test_as_mut_slice_single_entry() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, i32>] {
            &mut self.entries
        }

        fn as_mut_slice(&mut self) -> &mut Slice<i32, i32> {
            Slice::from_mut_slice(self.as_entries_mut())
        }
    }

    let mut map = TestMap {
        entries: vec![Bucket {
            hash: HashValue::new(0),
            key: 1,
            value: 10,
        }],
    };
    
    let slice = map.as_mut_slice();
    assert_eq!(slice.entries.len(), 1);
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[0].value, 10);
}

#[test]
fn test_as_mut_slice_multiple_entries() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, i32>] {
            &mut self.entries
        }

        fn as_mut_slice(&mut self) -> &mut Slice<i32, i32> {
            Slice::from_mut_slice(self.as_entries_mut())
        }
    }

    let mut map = TestMap {
        entries: vec![
            Bucket {
                hash: HashValue::new(1),
                key: 1,
                value: 10,
            },
            Bucket {
                hash: HashValue::new(2),
                key: 2,
                value: 20,
            },
        ],
    };

    let slice = map.as_mut_slice();
    assert_eq!(slice.entries.len(), 2);
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[0].value, 10);
    assert_eq!(slice.entries[1].key, 2);
    assert_eq!(slice.entries[1].value, 20);
}

