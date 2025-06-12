// Answer 0

#[test]
#[should_panic(expected = "range start index 5 out of range for slice of length 5")]
fn test_simplify_range_start_index_out_of_bounds_included() {
    use std::ops::{Range, Bound, RangeBounds};
    
    pub(crate) fn simplify_range<R>(range: R, len: usize) -> Range<usize>
    where
        R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) if i <= len => i,
            Bound::Excluded(&i) if i < len => i + 1,
            Bound::Included(i) | Bound::Excluded(i) => {
                panic!("range start index {i} out of range for slice of length {len}")
            }
        };
        let end = match range.end_bound() {
            Bound::Unbounded => len,
            Bound::Excluded(&i) if i <= len => i,
            Bound::Included(&i) if i < len => i + 1,
            Bound::Included(i) | Bound::Excluded(i) => {
                panic!("range end index {i} out of range for slice of length {len}")
            }
        };
        if start > end {
            panic!(
                "range start index {:?} should be <= range end index {:?}",
                range.start_bound(),
                range.end_bound()
            );
        }
        start..end
    }
    
    let range = 5..10; // panic expected here, as 5 is included and should be <= 5
    let len = 5;
    simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range end index 6 out of range for slice of length 5")]
fn test_simplify_range_end_index_out_of_bounds_included() {
    use std::ops::{Range, Bound, RangeBounds};

    pub(crate) fn simplify_range<R>(range: R, len: usize) -> Range<usize>
    where
        R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) if i <= len => i,
            Bound::Excluded(&i) if i < len => i + 1,
            Bound::Included(i) | Bound::Excluded(i) => {
                panic!("range start index {i} out of range for slice of length {len}")
            }
        };
        let end = match range.end_bound() {
            Bound::Unbounded => len,
            Bound::Excluded(&i) if i <= len => i,
            Bound::Included(&i) if i < len => i + 1,
            Bound::Included(i) | Bound::Excluded(i) => {
                panic!("range end index {i} out of range for slice of length {len}")
            }
        };
        if start > end {
            panic!(
                "range start index {:?} should be <= range end index {:?}",
                range.start_bound(),
                range.end_bound()
            );
        }
        start..end
    }

    let range = 0..=6; // panic expected here, as last included index 6 is out of bounds for len 5
    let len = 5;
    simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range start index 10 out of range for slice of length 5")]
fn test_simplify_range_start_index_out_of_bounds_excluded() {
    use std::ops::{Range, Bound, RangeBounds};

    pub(crate) fn simplify_range<R>(range: R, len: usize) -> Range<usize>
    where
        R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) if i <= len => i,
            Bound::Excluded(&i) if i < len => i + 1,
            Bound::Included(i) | Bound::Excluded(i) => {
                panic!("range start index {i} out of range for slice of length {len}")
            }
        };
        let end = match range.end_bound() {
            Bound::Unbounded => len,
            Bound::Excluded(&i) if i <= len => i,
            Bound::Included(&i) if i < len => i + 1,
            Bound::Included(i) | Bound::Excluded(i) => {
                panic!("range end index {i} out of range for slice of length {len}")
            }
        };
        if start > end {
            panic!(
                "range start index {:?} should be <= range end index {:?}",
                range.start_bound(),
                range.end_bound()
            );
        }
        start..end
    }

    let range = 10..15; // panic expected here, 10 is excluded, no issue here
    let len = 5;
    simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range end index 5 out of range for slice of length 5")]
fn test_simplify_range_end_index_out_of_bounds_excluded() {
    use std::ops::{Range, Bound, RangeBounds};

    pub(crate) fn simplify_range<R>(range: R, len: usize) -> Range<usize>
    where
        R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) if i <= len => i,
            Bound::Excluded(&i) if i < len => i + 1,
            Bound::Included(i) | Bound::Excluded(i) => {
                panic!("range start index {i} out of range for slice of length {len}")
            }
        };
        let end = match range.end_bound() {
            Bound::Unbounded => len,
            Bound::Excluded(&i) if i <= len => i,
            Bound::Included(&i) if i < len => i + 1,
            Bound::Included(i) | Bound::Excluded(i) => {
                panic!("range end index {i} out of range for slice of length {len}")
            }
        };
        if start > end {
            panic!(
                "range start index {:?} should be <= range end index {:?}",
                range.start_bound(),
                range.end_bound()
            );
        }
        start..end
    }

    let range = 0..5; // panic expected here, as 5 is the excluded end index and out of bounds for len 5
    let len = 5;
    simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range start index 6 should be <= range end index 5")]
fn test_simplify_range_start_greater_than_end() {
    use std::ops::{Range, Bound, RangeBounds};

    pub(crate) fn simplify_range<R>(range: R, len: usize) -> Range<usize>
    where
        R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) if i <= len => i,
            Bound::Excluded(&i) if i < len => i + 1,
            Bound::Included(i) | Bound::Excluded(i) => {
                panic!("range start index {i} out of range for slice of length {len}")
            }
        };
        let end = match range.end_bound() {
            Bound::Unbounded => len,
            Bound::Excluded(&i) if i <= len => i,
            Bound::Included(&i) if i < len => i + 1,
            Bound::Included(i) | Bound::Excluded(i) => {
                panic!("range end index {i} out of range for slice of length {len}")
            }
        };
        if start > end {
            panic!(
                "range start index {:?} should be <= range end index {:?}",
                range.start_bound(),
                range.end_bound()
            );
        }
        start..end
    }
    
    let range = 6..5; // panic expected here as start is greater than end 
    let len = 5;
    simplify_range(range, len);
}

