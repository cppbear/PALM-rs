// Answer 0

#[test]
fn test_get_range_valid() {
    use std::ops::Range;

    struct MyIndexMap<T> {
        entries: Vec<T>,
    }

    impl<T> MyIndexMap<T> {
        pub fn new(entries: Vec<T>) -> Self {
            MyIndexMap { entries }
        }

        pub fn get_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<&[T]> {
            let len = self.entries.len();
            let range = try_simplify_range(range, len)?;
            Some(&self.entries[range])
        }
    }

    fn try_simplify_range<R: RangeBounds<usize>>(range: R, len: usize) -> Option<Range<usize>> {
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

        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let map = MyIndexMap::new(vec![1, 2, 3, 4, 5]);
    let result = map.get_range(1..4);
    assert_eq!(result, Some(&[2, 3, 4]));
}

#[test]
fn test_get_range_empty() {
    use std::ops::Range;

    struct MyIndexMap<T> {
        entries: Vec<T>,
    }

    impl<T> MyIndexMap<T> {
        pub fn new(entries: Vec<T>) -> Self {
            MyIndexMap { entries }
        }

        pub fn get_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<&[T]> {
            let len = self.entries.len();
            let range = try_simplify_range(range, len)?;
            Some(&self.entries[range])
        }
    }

    fn try_simplify_range<R: RangeBounds<usize>>(range: R, len: usize) -> Option<Range<usize>> {
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

        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let map: MyIndexMap<i32> = MyIndexMap::new(vec![]);
    let result = map.get_range(0..1);
    assert_eq!(result, None);
}

#[test]
fn test_get_range_invalid() {
    use std::ops::Range;

    struct MyIndexMap<T> {
        entries: Vec<T>,
    }

    impl<T> MyIndexMap<T> {
        pub fn new(entries: Vec<T>) -> Self {
            MyIndexMap { entries }
        }

        pub fn get_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<&[T]> {
            let len = self.entries.len();
            let range = try_simplify_range(range, len)?;
            Some(&self.entries[range])
        }
    }

    fn try_simplify_range<R: RangeBounds<usize>>(range: R, len: usize) -> Option<Range<usize>> {
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

        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let map = MyIndexMap::new(vec![1, 2, 3]);
    let result = map.get_range(3..5);
    assert_eq!(result, None);
}

#[test]
fn test_get_range_out_of_bounds() {
    use std::ops::Range;

    struct MyIndexMap<T> {
        entries: Vec<T>,
    }

    impl<T> MyIndexMap<T> {
        pub fn new(entries: Vec<T>) -> Self {
            MyIndexMap { entries }
        }

        pub fn get_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<&[T]> {
            let len = self.entries.len();
            let range = try_simplify_range(range, len)?;
            Some(&self.entries[range])
        }
    }

    fn try_simplify_range<R: RangeBounds<usize>>(range: R, len: usize) -> Option<Range<usize>> {
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

        if start < end && end <= len {
            Some(start..end)
        } else {
            None
        }
    }

    let map = MyIndexMap::new(vec![1, 2, 3]);
    let result = map.get_range(1..4);
    assert_eq!(result, Some(&[2, 3]));
}

