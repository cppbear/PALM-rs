// Answer 0

#[test]
fn test_get_range_valid() {
    struct Slice {
        entries: Vec<(usize, usize)>
    }

    impl Slice {
        pub fn from_slice(slice: &[(usize, usize)]) -> &Self {
            // Implementation is not necessary for the test, returning a new Slice for the sake of having a valid method
            unsafe { &*(slice as *const _ as *const Self) }
        }

        pub fn len(&self) -> usize {
            self.entries.len()
        }

        pub fn get(&self, range: std::ops::Range<usize>) -> Option<&[(usize, usize)]> {
            if range.start < self.len() && range.end <= self.len() {
                Some(&self.entries[range])
            } else {
                None
            }
        }
    }

    fn try_simplify_range<R: RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            Bound::Included(&v) => v,
            Bound::Excluded(&v) => v + 1,
            Bound::Unbound => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&v) => v + 1,
            Bound::Excluded(&v) => v,
            Bound::Unbound => len,
        };

        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let slice = Slice {
        entries: vec![(0, 0), (1, 1), (2, 2)],
    };

    let result = slice.get_range(0..2);
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_get_range_empty() {
    struct Slice {
        entries: Vec<(usize, usize)>
    }

    impl Slice {
        pub fn from_slice(slice: &[(usize, usize)]) -> &Self {
            unsafe { &*(slice as *const _ as *const Self) }
        }

        pub fn len(&self) -> usize {
            self.entries.len()
        }

        pub fn get(&self, range: std::ops::Range<usize>) -> Option<&[(usize, usize)]> {
            if range.start < self.len() && range.end <= self.len() {
                Some(&self.entries[range])
            } else {
                None
            }
        }
    }

    fn try_simplify_range<R: RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            Bound::Included(&v) => v,
            Bound::Excluded(&v) => v + 1,
            Bound::Unbound => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&v) => v + 1,
            Bound::Excluded(&v) => v,
            Bound::Unbound => len,
        };

        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let slice = Slice {
        entries: vec![],
    };

    let result = slice.get_range(0..1);
    assert!(result.is_none());
}

#[test]
fn test_get_range_invalid() {
    struct Slice {
        entries: Vec<(usize, usize)>
    }

    impl Slice {
        pub fn from_slice(slice: &[(usize, usize)]) -> &Self {
            unsafe { &*(slice as *const _ as *const Self) }
        }

        pub fn len(&self) -> usize {
            self.entries.len()
        }

        pub fn get(&self, range: std::ops::Range<usize>) -> Option<&[(usize, usize)]> {
            if range.start < self.len() && range.end <= self.len() {
                Some(&self.entries[range])
            } else {
                None
            }
        }
    }

    fn try_simplify_range<R: RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            Bound::Included(&v) => v,
            Bound::Excluded(&v) => v + 1,
            Bound::Unbound => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&v) => v + 1,
            Bound::Excluded(&v) => v,
            Bound::Unbound => len,
        };

        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let slice = Slice {
        entries: vec![(0, 0)],
    };

    let result = slice.get_range(1..3);
    assert!(result.is_none());
}

