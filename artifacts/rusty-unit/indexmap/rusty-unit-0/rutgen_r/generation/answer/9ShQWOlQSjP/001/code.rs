// Answer 0

#[test]
fn test_get_range_mut_none_due_to_empty_entries() {
    struct Slice {
        entries: Vec<(usize, usize)>,
    }

    impl Slice {
        pub fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Self> {
            let range = try_simplify_range(range, self.entries.len())?;
            self.entries.get_mut(range).map(Slice::from_mut_slice)
        }

        pub fn from_mut_slice(slice: &mut [(usize, usize)]) -> &mut Self {
            unsafe { &mut *(slice as *mut _ as *mut Self) }
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&v) => v,
            std::ops::Bound::Excluded(&v) => v + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(&v) => v + 1,
            std::ops::Bound::Excluded(&v) => v,
            std::ops::Bound::Unbounded => len,
        };

        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let mut slice = Slice { entries: vec![] };
    assert_eq!(slice.get_range_mut(0..1), None);
}

#[test]
fn test_get_range_mut_none_due_to_negative_indices() {
    struct Slice {
        entries: Vec<(usize, usize)>,
    }

    impl Slice {
        pub fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Self> {
            let range = try_simplify_range(range, self.entries.len())?;
            self.entries.get_mut(range).map(Slice::from_mut_slice)
        }

        pub fn from_mut_slice(slice: &mut [(usize, usize)]) -> &mut Self {
            unsafe { &mut *(slice as *mut _ as *mut Self) }
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&v) => v,
            std::ops::Bound::Excluded(&v) => v + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(&v) => v + 1,
            std::ops::Bound::Excluded(&v) => v,
            std::ops::Bound::Unbounded => len,
        };

        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let mut slice = Slice { entries: vec![1, 2] };
    assert_eq!(slice.get_range_mut(-1..1), None);
}

#[test]
fn test_get_range_mut_none_due_to_out_of_bounds() {
    struct Slice {
        entries: Vec<(usize, usize)>,
    }

    impl Slice {
        pub fn get_range_mut<R: std::ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Self> {
            let range = try_simplify_range(range, self.entries.len())?;
            self.entries.get_mut(range).map(Slice::from_mut_slice)
        }

        pub fn from_mut_slice(slice: &mut [(usize, usize)]) -> &mut Self {
            unsafe { &mut *(slice as *mut _ as *mut Self) }
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&v) => v,
            std::ops::Bound::Excluded(&v) => v + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(&v) => v + 1,
            std::ops::Bound::Excluded(&v) => v,
            std::ops::Bound::Unbounded => len,
        };

        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let mut slice = Slice { entries: vec![1, 2] };
    assert_eq!(slice.get_range_mut(2..4), None);
}

