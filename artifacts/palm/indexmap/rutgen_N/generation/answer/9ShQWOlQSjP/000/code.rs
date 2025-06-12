// Answer 0

#[test]
fn test_get_range_mut_with_valid_range() {
    struct Slice {
        entries: Vec<(i32, i32)>,
    }

    impl Slice {
        fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Self> {
            let range = try_simplify_range(range, self.entries.len())?;
            self.entries.get_mut(range).map(Slice::from_mut_slice)
        }
        
        fn from_mut_slice(entries: &mut [(i32, i32)]) -> &mut Self {
            unsafe { &mut *(entries as *mut _ as *mut Slice) }
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&n) => n,
            std::ops::Bound::Excluded(&n) => n + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&n) => n + 1,
            std::ops::Bound::Excluded(&n) => n,
            std::ops::Bound::Unbounded => len,
        };
        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let mut slice = Slice {
        entries: vec![(1, 2), (3, 4), (5, 6)],
    };

    let result = slice.get_range_mut(0..2);
    assert!(result.is_some());
}

#[test]
fn test_get_range_mut_with_empty_range() {
    struct Slice {
        entries: Vec<(i32, i32)>,
    }

    impl Slice {
        fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Self> {
            let range = try_simplify_range(range, self.entries.len())?;
            self.entries.get_mut(range).map(Slice::from_mut_slice)
        }
        
        fn from_mut_slice(entries: &mut [(i32, i32)]) -> &mut Self {
            unsafe { &mut *(entries as *mut _ as *mut Slice) }
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&n) => n,
            std::ops::Bound::Excluded(&n) => n + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&n) => n + 1,
            std::ops::Bound::Excluded(&n) => n,
            std::ops::Bound::Unbounded => len,
        };
        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let mut slice = Slice {
        entries: vec![(1, 2), (3, 4), (5, 6)],
    };

    let result = slice.get_range_mut(2..2);
    assert!(result.is_none());
}

#[test]
fn test_get_range_mut_with_invalid_range() {
    struct Slice {
        entries: Vec<(i32, i32)>,
    }

    impl Slice {
        fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Self> {
            let range = try_simplify_range(range, self.entries.len())?;
            self.entries.get_mut(range).map(Slice::from_mut_slice)
        }
        
        fn from_mut_slice(entries: &mut [(i32, i32)]) -> &mut Self {
            unsafe { &mut *(entries as *mut _ as *mut Slice) }
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&n) => n,
            std::ops::Bound::Excluded(&n) => n + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&n) => n + 1,
            std::ops::Bound::Excluded(&n) => n,
            std::ops::Bound::Unbounded => len,
        };
        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let mut slice = Slice {
        entries: vec![(1, 2), (3, 4), (5, 6)],
    };

    let result = slice.get_range_mut(1..5);
    assert!(result.is_none());
}

#[test]
fn test_get_range_mut_with_unbounded_range() {
    struct Slice {
        entries: Vec<(i32, i32)>,
    }

    impl Slice {
        fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Self> {
            let range = try_simplify_range(range, self.entries.len())?;
            self.entries.get_mut(range).map(Slice::from_mut_slice)
        }
        
        fn from_mut_slice(entries: &mut [(i32, i32)]) -> &mut Self {
            unsafe { &mut *(entries as *mut _ as *mut Slice) }
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&n) => n,
            std::ops::Bound::Excluded(&n) => n + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&n) => n + 1,
            std::ops::Bound::Excluded(&n) => n,
            std::ops::Bound::Unbounded => len,
        };
        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let mut slice = Slice {
        entries: vec![(1, 2), (3, 4), (5, 6)],
    };

    let result = slice.get_range_mut(..);
    assert!(result.is_some());
}

