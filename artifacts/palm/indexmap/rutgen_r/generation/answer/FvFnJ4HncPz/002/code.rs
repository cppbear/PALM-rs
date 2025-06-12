// Answer 0

#[test]
fn test_get_range_mut_valid_range() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut Vec<(usize, usize)> {
            &mut self.entries
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut [usize; 2]> {
            let entries = self.as_entries_mut();
            let range = try_simplify_range(range, entries.len())?;
            entries.get_mut(range).map(|s| {
                let start = entries[range.start()].0;
                let end = entries[range.end() - 1].1;
                &mut [start, end]
            })
        }

        fn new(entries: Vec<(usize, usize)>) -> Self {
            TestMap { entries }
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => len,
        };
        if start <= end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let mut map = TestMap::new(vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let result = map.get_range_mut(0..2);
    assert!(result.is_some());
    let range = result.unwrap();
    assert_eq!(range[0], 0);
    assert_eq!(range[1], 2);
}

#[test]
#[should_panic]
fn test_get_range_mut_invalid_range_start_greater_than_end() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut Vec<(usize, usize)> {
            &mut self.entries
        }

        fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut [usize; 2]> {
            let entries = self.as_entries_mut();
            let range = try_simplify_range(range, entries.len())?;
            entries.get_mut(range).map(|s| {
                let start = entries[range.start()].0;
                let end = entries[range.end() - 1].1;
                &mut [start, end]
            })
        }

        fn new(entries: Vec<(usize, usize)>) -> Self {
            TestMap { entries }
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => len,
        };
        if start <= end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let mut map = TestMap::new(vec![(0, 1), (1, 2)]);
    let _ = map.get_range_mut(2..1); // This should panic
}

#[test]
fn test_get_range_mut_exclusive_end() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut Vec<(usize, usize)> {
            &mut self.entries
        }

        fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut [usize; 2]> {
            let entries = self.as_entries_mut();
            let range = try_simplify_range(range, entries.len())?;
            entries.get_mut(range).map(|s| {
                let start = entries[range.start()].0;
                let end = entries[range.end() - 1].1;
                &mut [start, end]
            })
        }

        fn new(entries: Vec<(usize, usize)>) -> Self {
            TestMap { entries }
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => len,
        };
        if start <= end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let mut map = TestMap::new(vec![(0, 1), (1, 2), (2, 3)]);
    let result = map.get_range_mut(1..3);
    assert!(result.is_some());
    let range = result.unwrap();
    assert_eq!(range[0], 1);
    assert_eq!(range[1], 3);
}

