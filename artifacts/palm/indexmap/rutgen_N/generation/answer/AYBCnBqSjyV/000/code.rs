// Answer 0

#[test]
fn test_get_range_valid_range() {
    struct TestStruct {
        entries: Vec<i32>,
    }

    impl TestStruct {
        pub fn get_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<&[i32]> {
            let range = self.entries.get_range(range)?;
            Some(&self.entries[range])
        }
    }

    let data = TestStruct { entries: vec![1, 2, 3, 4, 5] };
    assert_eq!(data.get_range(0..3), Some(&[1, 2, 3][..]));
    assert_eq!(data.get_range(1..4), Some(&[2, 3, 4][..]));
}

#[test]
fn test_get_range_out_of_bounds() {
    struct TestStruct {
        entries: Vec<i32>,
    }

    impl TestStruct {
        pub fn get_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<&[i32]> {
            let range = self.entries.get_range(range)?;
            Some(&self.entries[range])
        }
    }

    let data = TestStruct { entries: vec![1, 2, 3, 4, 5] };
    assert_eq!(data.get_range(0..10), None);
    assert_eq!(data.get_range(5..5), Some(&[][..]));
}

#[test]
fn test_get_range_empty_slice() {
    struct TestStruct {
        entries: Vec<i32>,
    }

    impl TestStruct {
        pub fn get_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Option<&[i32]> {
            let range = self.entries.get_range(range)?;
            Some(&self.entries[range])
        }
    }

    let data = TestStruct { entries: vec![] };
    assert_eq!(data.get_range(0..1), None);
    assert_eq!(data.get_range(0..0), Some(&[][..]));
}

