// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::RangeBounds;

    struct TestCollection {
        data: Vec<i32>,
    }

    impl TestCollection {
        fn new(data: Vec<i32>) -> Self {
            TestCollection { data }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn as_entries(&self) -> &[i32] {
            &self.data
        }

        pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&[i32]> {
            let entries = self.as_entries();
            let (start, end) = match (range.start_bound(), range.end_bound()) {
                (Bound::Included(&start), Bound::Excluded(&end)) => (start, end),
                (Bound::Excluded(&start), Bound::Included(&end)) => (start + 1, end + 1),
                (Bound::Included(&start), Bound::Included(&end)) => (start, end + 1),
                (Bound::Excluded(&start), Bound::Excluded(&end)) => (start + 1, end),
                _ => return None,
            };

            if end <= entries.len() {
                Some(&entries[start..end])
            } else {
                None
            }
        }
    }

    #[test]
    fn test_get_range_valid_inclusive_to_exclusive() {
        let collection = TestCollection::new(vec![1, 2, 3, 4, 5]);
        let result = collection.get_range(0..3);
        assert_eq!(result, Some(&[1, 2, 3]));
    }

    #[test]
    fn test_get_range_valid_inclusive_to_inclusive() {
        let collection = TestCollection::new(vec![10, 20, 30, 40, 50]);
        let result = collection.get_range(1..=3);
        assert_eq!(result, Some(&[20, 30, 40]));
    }

    #[test]
    fn test_get_range_empty_slice() {
        let collection = TestCollection::new(vec![]);
        let result = collection.get_range(0..0);
        assert_eq!(result, Some(&[]));
    }

    #[test]
    fn test_get_range_out_of_bounds() {
        let collection = TestCollection::new(vec![1, 2, 3]);
        let result = collection.get_range(2..10);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_range_negative_start() {
        let collection = TestCollection::new(vec![1, 2, 3]);
        let result = collection.get_range(usize::MAX..3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_range_start_equals_length() {
        let collection = TestCollection::new(vec![1, 2, 3]);
        let result = collection.get_range(3..3);
        assert_eq!(result, Some(&[]));
    }

    #[test]
    fn test_get_range_end_equals_length() {
        let collection = TestCollection::new(vec![1, 2, 3]);
        let result = collection.get_range(0..3);
        assert_eq!(result, Some(&[1, 2, 3]));
    }
}

