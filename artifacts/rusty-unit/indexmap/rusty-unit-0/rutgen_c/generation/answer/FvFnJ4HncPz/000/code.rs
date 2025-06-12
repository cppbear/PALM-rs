// Answer 0

#[test]
fn test_get_range_mut_valid_range() {
    struct MyMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl MyMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, String>] {
            &mut self.entries
        }

        fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Slice<i32, String>> {
            let entries = self.as_entries_mut();
            let range = try_simplify_range(range, entries.len())?;
            entries.get_mut(range).map(Slice::from_mut_slice)
        }
    }

    let mut map = MyMap {
        entries: vec![
            Bucket { hash: HashValue::new(1), key: 1, value: "one".to_string() },
            Bucket { hash: HashValue::new(2), key: 2, value: "two".to_string() },
            Bucket { hash: HashValue::new(3), key: 3, value: "three".to_string() },
        ],
    };

    let slice = map.get_range_mut(0..2);
    assert!(slice.is_some());
}

#[test]
fn test_get_range_mut_invalid_range() {
    struct MyMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl MyMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, String>] {
            &mut self.entries
        }

        fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Slice<i32, String>> {
            let entries = self.as_entries_mut();
            let range = try_simplify_range(range, entries.len())?;
            entries.get_mut(range).map(Slice::from_mut_slice)
        }
    }

    let mut map = MyMap {
        entries: vec![
            Bucket { hash: HashValue::new(1), key: 1, value: "one".to_string() },
            Bucket { hash: HashValue::new(2), key: 2, value: "two".to_string() },
        ],
    };

    let slice = map.get_range_mut(2..4);
    assert!(slice.is_none());
}

#[test]
fn test_get_range_mut_empty_map() {
    struct MyMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl MyMap {
        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, String>] {
            &mut self.entries
        }

        fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Slice<i32, String>> {
            let entries = self.as_entries_mut();
            let range = try_simplify_range(range, entries.len())?;
            entries.get_mut(range).map(Slice::from_mut_slice)
        }
    }

    let mut map = MyMap { entries: vec![] };
    let slice = map.get_range_mut(0..1);
    assert!(slice.is_none());
}

