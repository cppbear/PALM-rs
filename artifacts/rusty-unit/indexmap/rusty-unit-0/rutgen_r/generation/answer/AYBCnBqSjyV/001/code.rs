// Answer 0

#[test]
fn test_get_range_empty_slice() {
    struct TestStruct {
        entries: Vec<i32>,
    }

    impl TestStruct {
        pub fn get_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<&[i32]> {
            let range = try_simplify_range(range, self.entries.len())?;
            self.entries.get(range).map(|s| &self.entries[s])
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&v) => v,
            std::ops::Bound::Excluded(&v) => v.checked_add(1)?,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&v) => v.checked_add(1)?,
            std::ops::Bound::Excluded(&v) => v,
            std::ops::Bound::Unbounded => len,
        };
        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let test_struct = TestStruct { entries: Vec::new() };

    // Case 1: Invalid range (negative indices)
    assert_eq!(test_struct.get_range(1..0), None);

    // Case 2: Invalid range (out of bounds)
    assert_eq!(test_struct.get_range(0..2), None);
}

#[test]
fn test_get_range_full_slice() {
    struct TestStruct {
        entries: Vec<i32>,
    }

    impl TestStruct {
        pub fn get_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<&[i32]> {
            let range = try_simplify_range(range, self.entries.len())?;
            self.entries.get(range).map(|s| &self.entries[s])
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&v) => v,
            std::ops::Bound::Excluded(&v) => v.checked_add(1)?,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&v) => v.checked_add(1)?,
            std::ops::Bound::Excluded(&v) => v,
            std::ops::Bound::Unbounded => len,
        };
        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let test_struct = TestStruct { entries: vec![1, 2, 3, 4, 5] };

    // Case 1: Valid range that covers the entire slice
    assert_eq!(test_struct.get_range(..), Some(&[1, 2, 3, 4, 5][..]));

    // Case 2: Valid range (full slice via inclusive range)
    assert_eq!(test_struct.get_range(0..5), Some(&[1, 2, 3, 4, 5][..]));
}

#[test]
fn test_get_range_partial_slice() {
    struct TestStruct {
        entries: Vec<i32>,
    }

    impl TestStruct {
        pub fn get_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<&[i32]> {
            let range = try_simplify_range(range, self.entries.len())?;
            self.entries.get(range).map(|s| &self.entries[s])
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&v) => v,
            std::ops::Bound::Excluded(&v) => v.checked_add(1)?,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&v) => v.checked_add(1)?,
            std::ops::Bound::Excluded(&v) => v,
            std::ops::Bound::Unbounded => len,
        };
        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let test_struct = TestStruct { entries: vec![10, 20, 30, 40, 50] };

    // Case 1: Valid range (partial slice)
    assert_eq!(test_struct.get_range(1..4), Some(&[20, 30, 40][..]));

    // Case 2: Valid range (just the first element)
    assert_eq!(test_struct.get_range(0..1), Some(&[10][..]));

    // Case 3: Valid range (last element)
    assert_eq!(test_struct.get_range(4..5), Some(&[50][..]));
}

