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

        fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut [(usize, usize)]> {
            use std::ops::Bound::*;
            let entries = self.as_entries_mut();
            let start = match range.start_bound() {
                Unbounded => 0,
                Included(&n) => n,
                Excluded(&n) => n + 1,
            };
            let end = match range.end_bound() {
                Unbounded => entries.len(),
                Included(&n) => n + 1,
                Excluded(&n) => n,
            };
            if start < end && end <= entries.len() {
                Some(&mut entries[start..end])
            } else {
                None
            }
        }
    }

    let mut map = TestMap {
        entries: vec![(0, 1), (2, 3), (4, 5)],
    };

    let result = map.get_range_mut(1..3);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), &mut vec![(2, 3), (4, 5)]);
}

#[test]
fn test_get_range_mut_out_of_bounds() {
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

        fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut [(usize, usize)]> {
            use std::ops::Bound::*;
            let entries = self.as_entries_mut();
            let start = match range.start_bound() {
                Unbounded => 0,
                Included(&n) => n,
                Excluded(&n) => n + 1,
            };
            let end = match range.end_bound() {
                Unbounded => entries.len(),
                Included(&n) => n + 1,
                Excluded(&n) => n,
            };
            if start < end && end <= entries.len() {
                Some(&mut entries[start..end])
            } else {
                None
            }
        }
    }

    let mut map = TestMap {
        entries: vec![(0, 1), (2, 3)],
    };

    let result = map.get_range_mut(2..4);
    assert!(result.is_none());
}

#[test]
fn test_get_range_mut_empty_range() {
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

        fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut [(usize, usize)]> {
            use std::ops::Bound::*;
            let entries = self.as_entries_mut();
            let start = match range.start_bound() {
                Unbounded => 0,
                Included(&n) => n,
                Excluded(&n) => n + 1,
            };
            let end = match range.end_bound() {
                Unbounded => entries.len(),
                Included(&n) => n + 1,
                Excluded(&n) => n,
            };
            if start < end && end <= entries.len() {
                Some(&mut entries[start..end])
            } else {
                None
            }
        }
    }

    let mut map = TestMap {
        entries: vec![(0, 1)],
    };

    let result = map.get_range_mut(1..1);
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_get_range_mut_unbounded_range() {
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

        fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut [(usize, usize)]> {
            use std::ops::Bound::*;
            let entries = self.as_entries_mut();
            let start = match range.start_bound() {
                Unbounded => 0,
                Included(&n) => n,
                Excluded(&n) => n + 1,
            };
            let end = match range.end_bound() {
                Unbounded => entries.len(),
                Included(&n) => n + 1,
                Excluded(&n) => n,
            };
            if start < end && end <= entries.len() {
                Some(&mut entries[start..end])
            } else {
                None
            }
        }
    }

    let mut map = TestMap {
        entries: vec![(0, 1), (2, 3), (4, 5)],
    };

    let result = map.get_range_mut(..2);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), &mut [(0, 1), (2, 3)]);
}

