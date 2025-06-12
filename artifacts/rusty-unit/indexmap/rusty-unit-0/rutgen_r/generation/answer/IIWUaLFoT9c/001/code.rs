// Answer 0

#[test]
fn test_get_range_none_due_to_invalid_range() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn as_entries(&self) -> &[i32] {
            &self.data
        }

        fn len(&self) -> usize {
            self.data.len()
        }
        
        pub fn get_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<&[i32]> {
            let entries = self.as_entries();
            let range = try_simplify_range(range, entries.len())?;
            entries.get(range).map(|slice| &entries[slice.start..slice.end])
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s.saturating_add(1),
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => e.saturating_add(1),
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => len,
        };
        
        if start < len && end <= len && start < end {
            Some(start..end)
        } else {
            None
        }
    }

    let test_struct = TestStruct { data: vec![1, 2, 3] };

    assert!(test_struct.get_range(3..5).is_none()); // range starts beyond the length
    assert!(test_struct.get_range(2..2).is_none()); // range where start equals end
    assert!(test_struct.get_range(1..=3).is_none()); // range exceeds the length
} 

#[test]
fn test_get_range_none_due_to_empty_struct() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn as_entries(&self) -> &[i32] {
            &self.data
        }
        
        pub fn get_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<&[i32]> {
            let entries = self.as_entries();
            let range = try_simplify_range(range, entries.len())?;
            entries.get(range).map(|slice| &entries[slice.start..slice.end])
        }
    }

    fn try_simplify_range<R: std::ops::RangeBounds<usize>>(range: R, len: usize) -> Option<std::ops::Range<usize>> {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s.saturating_add(1),
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => e.saturating_add(1),
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => len,
        };

        if start < len && end <= len && start < end {
            Some(start..end)
        } else {
            None
        }
    }

    let test_struct = TestStruct { data: vec![] };

    assert!(test_struct.get_range(..).is_none()); // empty struct, should return None
}

